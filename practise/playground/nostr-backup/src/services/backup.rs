use std::fs::File;
use nostr_sdk::{Keys, prelude::*};
use crate::constants;
use crate::services::key_converter;


pub async fn create_backup_file(_followers: bool, _following: bool, key: &String, private_key: bool) {
    key_converter::decode_to_hex(key);
    // TODO: Cannot let to add the private key. Set clap
    // Create the keys
    let nostr_key = match private_key {
        true => Keys::from_sk_str(key).unwrap(),
        _ => Keys::from_pk_str(key).unwrap()
    };

    /*let options = Options::new().timeout(Some(Duration::from_secs(20)));
    let client = Client::with_opts(&nostr_key, options);*/
    
    let client = Client::new(&nostr_key);
    
    for relay in constants::get_relays().iter() {
        println!("Adding relay {} to the client...", relay);
        client.add_relay(relay, None).await.unwrap();
    }

    client.connect().await;

    let mut nostr_bech32_key = key.to_string();

    if !key_converter::bech32_checksum(key) {
        nostr_bech32_key = key_converter::encode_to_bech32("npub", key);
    }

    let _bech32 = XOnlyPublicKey::from_bech32(&nostr_bech32_key).expect("Cannot get the XOnlyPubKey");

    let author_vector = client.keys().public_key().to_string();

    println!("Waiting to subscription...");

    let subscription = Filter::new()
        .kind(Kind::ContactList)
        .author(author_vector);

    client.subscribe(vec![subscription]).await;
    

    println!("Subscribed!");

    let mut notifications = client.notifications();
    while let Ok(notification) = notifications.recv().await {
        // Process Events 
        if let RelayPoolNotification::Event(_url, event) = notification {
            // println!("Event URL: {}", url);
            save_file(event.tags);   
            // println!("\n\n");
            client.disconnect().await.unwrap();
        }
        // Process messages
        else if let RelayPoolNotification::Message(_url, message) = notification {
            // println!("Message URL: {:?}", url);
            // println!("{:?}", message::RelayMessage::Empty);
            if message::RelayMessage::Empty == message {
                println!("exit");
                break;
            }
        }
    }
    
    // Get the following peers
    // Get the followers peers
    // Create the JSON file
}

fn save_file(tags: Vec<Tag>) {
    let mut peers: Vec<String> = vec![];
    for pub_key in tags.into_iter() {
        peers.push(pub_key.as_vec()[1].clone());
    }
    let peers_json = serde_json::to_string(&peers).unwrap();
    File::create(constants::BACKUP_FILE).unwrap();
    std::fs::write(constants::BACKUP_FILE, peers_json).unwrap();

}

pub async fn set_meta_data(_key: &String) {
}