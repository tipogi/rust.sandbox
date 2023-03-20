use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let address = "127.0.0.1:6379";

    // Open a connection
    println!("Trying to connect to {:?}...", address);
    let mut client = client::connect(address).await?;
    println!("connected!");
    
    // Set and get the new key value
    client.set("hello", "Rusty".into()).await.unwrap();
    let result = client.get("hello").await?;

    // Wrap all the Bytes in a vector of u8
    let byte_vector = result.unwrap().into();
    // Convert byte vector into String
    let value = String::from_utf8(byte_vector).unwrap();

    println!("Got a value from server, {:?}", value);
    Ok(())
}
