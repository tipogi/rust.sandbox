use std::borrow::BorrowMut;
use std::pin::Pin;
use std::{sync::{Arc}, collections::HashMap};
use futures::Stream;
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{Request, Response, Status};
use std::io::{Error as IOError, ErrorKind as IOErrorKind};
use crate::dbm::DBM;
use crate::store::inventory_server::Inventory;
use crate::store::{
    Item, InventoryChangeResponse, ItemIdentifier, QuantityChangeRequest, InventoryUpdateResponse, PriceChangeRequest
};


const BAD_PRICE_ERR: &str = "provided PRICE was invalid";
const DUP_PRICE_ERR: &str = "item is already at this price";
const DUP_ITEM_ERR: &str = "item already exists in inventory";
const EMPTY_QUANT_ERR: &str = "invalid quantity of 0 provided";
const EMPTY_SKU_ERR: &str = "provided SKU was empty";
const NO_ID_ERR: &str = "no ID or SKU provided for item";
const NO_ITEM_ERR: &str = "the item requested was not found";
const NO_STOCK_ERR: &str = "no stock provided for item";
const UNSUFF_INV_ERR: &str = "not enough inventory for quantity change";
const INTERNAL_ERR: &str = "internal error";

#[derive(Debug)]
pub struct StoreInventory {
    inventory: Arc<Mutex<HashMap<String, Item>>>,
    dbm: Arc<Mutex<DBM>>
}

/*impl Default for StoreInventory {
    fn default() -> Self {
        StoreInventory {
            inventory: Arc::new(Mutex::new(HashMap::<String, Item>::new()))
        }
    }
}*/

impl StoreInventory {
    pub fn new(dbm: Arc<Mutex<DBM>>) -> Self {
        Self {
            inventory: Arc::new(Mutex::new(HashMap::<String, Item>::new())),
            dbm
        }
    }
}

// Implement Inventory trait which was generated from store.rs
// Add all the methods of the Inventory trait to StoreInventory
#[tonic::async_trait]
impl Inventory for StoreInventory {
    async fn add(
        &self,
        request: Request<Item>
    ) -> Result<Response<InventoryChangeResponse>, Status> {
        println!("RCP calling store ADD method...");
        // Consume self and return the object
        let item = request.into_inner();
        
        // validate SKU, verify that it's present and not empty
        let sku = match item.identifier.as_ref() {
            Some(id) if id.sku == "" => return Err(Status::invalid_argument(EMPTY_SKU_ERR)),
            Some(id) => id.sku.to_owned(),
            None => return Err(Status::invalid_argument(NO_ID_ERR))
        };

        // validate stock, verify its present and price is not negative or $0.00
        match item.stock.as_ref() {
            Some(stock) if (stock.price <= 0.00) => {
                return Err(Status::invalid_argument(BAD_PRICE_ERR))
            }
            Some(_) => {},
            None => return Err(Status::invalid_argument(NO_STOCK_ERR))
        }
        
        // Wait till we get the exclusive ownership over the DB
        // With that we lock mutex and we ensure thread safety and integrity
        let dbm = self.dbm.lock().await;
        let resp = dbm.exist_item(&sku).await;
        // Error control before create new Item
        match resp {
            Err(e) if e.kind() == IOErrorKind::NotFound => println!("The item does not exist, create new item. SKU: {:?}", &sku),
            Ok(_) => return Err(Status::already_exists(DUP_ITEM_ERR)),
            _ => return Err(Status::internal(INTERNAL_ERR))
        }

        dbm.create_item(item).await.unwrap();

        Ok(Response::new(InventoryChangeResponse {
            status: "success".into(),
        }))
    }

    async fn remove(
        &self, 
        request: Request<ItemIdentifier>
    ) -> Result<Response<InventoryChangeResponse>, Status> {
        println!("RCP calling store REMOVE method...");
        let identifier = request.into_inner();

        if identifier.sku == "" {
            return Err(Status::invalid_argument(EMPTY_SKU_ERR));
        }

        let dbm = self.dbm.lock().await;
        let msg = match dbm.remove_item(&identifier.sku).await {
            Some(_) => "success: Item was removed",
            None => "success: Item did not exist"
        };

        println!("Removed the product with sku={:?}", &identifier.sku);

        Ok(Response::new(InventoryChangeResponse {
            status: msg.into()
        }))
    }

    async fn get(
        &self,
        request: Request<ItemIdentifier>
    ) -> Result<Response<Item>, Status> {
        println!("RCP calling store GET method...");
        let identifier = request.into_inner();
        
        if identifier.sku == "" {
            return Err(Status::invalid_argument(EMPTY_SKU_ERR));
        }

        let dbm = self.dbm.lock().await;

        match dbm.exist_item(&identifier.sku).await {
            Ok(item) => return Ok(Response::new(Item::from(item))),
            _ => return Err(Status::not_found(NO_ITEM_ERR))
        }
    }

    async fn update_quantity(
        &self,
        request: Request<QuantityChangeRequest>
    ) -> Result<Response<InventoryUpdateResponse>, Status> {
        println!("RCP call to update_quantity method...");
        let mut new_items = request.into_inner();

        if new_items.sku == "" {
            return Err(Status::invalid_argument(EMPTY_SKU_ERR));
        }

        if new_items.change == 0 {
            return Err(Status::invalid_argument(EMPTY_QUANT_ERR));
        }

        let dbm = self.dbm.lock().await;

        // Retrieve the current inventory item data
        let item = match dbm.exist_item(&new_items.sku).await {
            Ok(item) => item,
            Err(_) => return Err(Status::not_found(NO_ITEM_ERR))
        };

        new_items.change  = match new_items.change {
            new_quantity if new_quantity < 0 => {
                if new_quantity.abs() as u32 > item.quantity {
                    return Err(Status::resource_exhausted(UNSUFF_INV_ERR))
                }
                // The new quantity is negative so, rest the new amount to the actual
                (item.quantity - new_quantity.abs() as u32) as i32
            }
            new_quantity => (item.quantity + new_quantity as u32) as i32
        };

        dbm.update_item_quantity(&new_items).await.unwrap();

        Ok(Response::new(InventoryUpdateResponse {
            status: "success".into(),
            price: item.price,
            quantity: new_items.change as u32
        }))
    }

    async fn update_price(
        &self,
        request: Request<PriceChangeRequest>
    ) -> Result<Response<InventoryUpdateResponse>, Status> {
        println!("RCP call to update_price method...");
        let update = request.into_inner();

        // don't allow empty SKU
        /*if update.sku == "" {
            return Err(Status::invalid_argument(EMPTY_SKU_ERR));
        }

        // $0.00 disallowed and negatives don't make sense, inform the user
        if update.price <= 0.0 {
            return Err(Status::invalid_argument(BAD_PRICE_ERR));
        }

        // retrieve the current inventory item data
        let mut map = self.inventory.lock().unwrap();
        let item = match map.get_mut(&update.sku) {
            Some(item) => item,
            None => return Err(Status::not_found(NO_ITEM_ERR)),
        };

        // retrieve the stock mutable so we can update the quantity
        let mut stock = match item.stock.borrow_mut() {
            Some(stock) => stock,
            None => return Err(Status::internal(NO_STOCK_ERR)),
        };

        // let the client know if they requested to change the price to the
        // price that is already currently set
        if stock.price == update.price {
            return Err(Status::invalid_argument(DUP_PRICE_ERR));
        }

        // update the item unit price
        stock.price = update.price;*/

        Ok(Response::new(InventoryUpdateResponse {
            status: "success".into(),
            price: 12.3,//stock.price,
            quantity: 1//stock.quantity,
        }))

    }

    type WatchStream = Pin<Box<dyn Stream<Item = Result<Item, Status>> + Send >>;

    async fn watch(
        &self,
        request: Request<ItemIdentifier>
    ) -> Result<Response<Self::WatchStream>, Status> {
        println!("RCP call to Watch method...");
        // retrieve the relevant item and get a baseline
        let item_identifier = request.into_inner();
        // Create a new request to get the item
        let mut item = self.get(Request::new(item_identifier.clone())).await?.into_inner();

        println!("Get item: {:?} with {:?} identifier", &item, &item_identifier);

        let (tx, rx) = mpsc::unbounded_channel();

        let inventory = self.inventory.clone();

        /*tokio::spawn(async move {
            loop {
                // Check every one second like this the loop is not going to be that intense
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                println!("Wake up after 1000 ms sleep");
                let map = inventory.lock().unwrap();
                let item_refresh = match map.get(&item_identifier.sku) {
                    Some(item) => item,
                    None => {
                        if let Err(err) = tx.send(Err(Status::not_found(NO_ITEM_ERR))) {
                            println!("ERROR: failed to update stream client: {:?}", err);
                        }
                        return;
                    }
                };

                // check to see if the item has changed since we last saw it,
                // and if it has inform the client via the stream.
                if item_refresh != &item {
                    println!("WATCH: Detected some changes in the item, message to the client");
                    if let Err(err) = tx.send(Ok(item_refresh.clone())) {
                        println!("ERROR: failed to update stream client: {:?}", err);
                        return;
                    } else {
                        println!("Some changes in the item!");
                    }
                }
                // cache the most recent copy of the item
                item = item_refresh.clone()
            }
        });*/

        println!("End of the tokio:spawn...");

        let stream = UnboundedReceiverStream::new(rx);
        Ok(Response::new(Box::pin(stream) as Self::WatchStream))
    }
}