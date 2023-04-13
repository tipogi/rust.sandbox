use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool, Pool, sqlite::SqlitePoolOptions};
use dotenvy::var;
use std::io::{Error as IOError, ErrorKind as IOErrorKind};
use sqlx::Error as SqlError;

use crate::{entites::{ItemEntity, ItemSku}, store::{Item, QuantityChangeRequest}};

/// DataBaseManager: Currently works for `SQLite`, disk database
#[derive(Debug)]
pub struct DBM {
    /// The database connection.
    connection: Pool<Sqlite>
}

impl DBM {
    pub async fn new() -> Self {
        let db_url = var("DATABASE_URL").expect("ERROR: DATABASE_URL is not set in the .env file");
        println!("Opening connection with sqlite database with name {:?}", db_url);
        if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
            println!("WARNING: The named database does not exist. Find the database migration file first before start up the server");
            DBM::create_database(&db_url).await;
        } else {
            println!("Database already exist, connecting to the db...")
        }
        let connection = SqlitePoolOptions::new()
            // I do not know if it does make sense since we wrap the connection in
            // a Arc struct
            .max_connections(10)
            .connect(&db_url)
            .await
            .unwrap();

        Self {
            connection
        }
    }

    async fn create_database(db_url: &String) {
        match Sqlite::create_database(db_url).await {
            Ok(_) => println!("Database created sucessfully!"),
            Err(e) => panic!("ERROR: It was an error in the creation of the DB: {:?}", e)
        }

        // Open a connection
        let connection = SqlitePool::connect(db_url).await.unwrap();

        // Create the migrations path
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let migrations = std::path::Path::new(&crate_dir).join("./migrations");

        let result = sqlx::migrate::Migrator::new(migrations)
            .await
            .unwrap()
            .run(&connection)
            .await;

        match result {
            Ok(_) => println!("Migration file imported successfully!"),
            Err(e) => panic!("ERROR: Migration error: {:?}", e)
        }
    }

    pub async fn exist_item(&self, sku: &String) -> Result<ItemEntity, IOError>{
        println!("searching SKU={:?}...", sku);
        let item = sqlx::query_as::<_,ItemEntity>(
            r#"SELECT * FROM items WHERE items.sku = ?1"#,
        )
        .bind(sku)
        .fetch_one(&self.connection)
        .await;
        
        match item {
            Ok(item_entity)   => return Ok(item_entity),
            Err(SqlError::RowNotFound) => return Err(IOError::new(IOErrorKind::NotFound, "The item does not exist")),
            Err(_)  => return Err(IOError::new(IOErrorKind::BrokenPipe, "Connection error"))
        }
    }

    pub async fn create_item(&self, item: Item) -> Result<ItemEntity, SqlError> {
        let stock = item.stock.unwrap();
        let response = sqlx::query_as::<_, ItemEntity>(
            r#"
                INSERT INTO items (sku, price, quantity, name, description)
                VALUES (?1, ?2, ?3, ?4, ?5)
                RETURNING *
            "#
        )
        .bind(&item.identifier.unwrap().sku)
        .bind(stock.price as f32)
        .bind(stock.quantity as u32)
        .bind::<&str>(item.information.as_ref().unwrap().name.as_ref())
        .bind::<&str>(item.information.as_ref().unwrap().description.as_ref())
        .fetch_one(&self.connection)
        .await;

        response
    }

    pub async fn remove_item(&self, sku: &String) -> Option<String> {
        let response = sqlx::query_as::<_,ItemSku>(
            r#"
                DELETE FROM items 
                WHERE items.sku = ?1
                RETURNING sku
            "#
        )
        .bind(sku)
        .fetch_one(&self.connection)
        .await;

        match response {
            Ok(_) => return Some(String::from("DELETED")),
            Err(e) => {
                println!("ERROR: {:?}", e);
                return None
            }
        }
    }

    pub async fn update_item_quantity(&self, new_item: &QuantityChangeRequest) -> Result<ItemSku, SqlError> {
        let response = sqlx::query_as::<_, ItemSku>(
            r#"
                UPDATE items
                SET quantity = ?1
                WHERE items.sku = ?2
                RETURNING sku
            "#
        )
        .bind(&new_item.change)
        .bind(&new_item.sku)
        .fetch_one(&self.connection)
        .await;

        response
    }
}