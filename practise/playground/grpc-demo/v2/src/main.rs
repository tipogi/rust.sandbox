use std::sync::{ Arc };
use tokio::sync::Mutex;
use v2::server;
use v2::store;
use v2::dbm::DBM;

use tonic::transport::Server;

use server::StoreInventory;
use store::inventory_server::InventoryServer;


mod store_proto {
    include!("store.rs");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("store_descriptor");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let dbm = DBM::new().await;
    let addr = "127.0.0.1:9001".parse()?;

    // IMPORTANT: The mutex it has to be async one (tokio). If not when we lock and after 
    // we make a query (await), we increase the risk for a deadlock in the working thread
    let arc_dbm = Arc::new(Mutex::new(dbm));

    let inventory = StoreInventory::new(arc_dbm.clone());

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(store_proto::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .add_service(InventoryServer::new(inventory))
        // With that in insomnia we would be able to get all the queries that
        // we can do against the server
        .add_service(reflection_service)
        .serve(addr)
        .await?;
    Ok(())
}
