use sqlx::FromRow;

use crate::store::{Item, ItemIdentifier, ItemStock, ItemInformation};

#[derive(Debug, FromRow)]
pub struct ItemEntity {
    pub sku: String,
    pub price: f32,
    pub quantity: u32,
    pub name: String,
    pub description: String
}

#[derive(Debug, FromRow)]
pub struct ItemSku {
    pub sku: String
}

impl From<ItemEntity> for Item {
    fn from(item_entity: ItemEntity) -> Self {
        Self {
            identifier: Some(ItemIdentifier {
                sku: item_entity.sku
            }),
            stock: Some(ItemStock {
                price: item_entity.price,
                quantity: item_entity.quantity,
            }),
            information: Some(ItemInformation {
                name: item_entity.name,
                description: item_entity.description
            })
        }
    }
}