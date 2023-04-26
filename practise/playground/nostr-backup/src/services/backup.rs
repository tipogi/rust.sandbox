use nostr_sdk::{Keys, prelude::*};
use crate::constants;
use crate::enums::Prefix;
use crate::services::key_converter;
use crate::structs::peers::Peers;


pub async fn create_backup_file(key: &String, followers: bool, following: bool) {
    
    // Before make the backup, check if the bech32 and HEX key are correct ones
    // Also if our key is in HEX format, encode to bech32 format
    let (npub_key, _hex_key) = key_converter::display_key_info(&key, Prefix::Npub);

    let nostr_key = Keys::from_pk_str(&npub_key).unwrap();
    let client = Client::new(&nostr_key);
    
    for relay in constants::get_relays().iter() {
        println!("Adding relay {} to the client...", relay);
        client.add_relay(relay, None).await.unwrap();
    }

    client.connect().await;

    let mut peers_backup = Peers::new();

    println!("Waiting to subscription...");

    if following {
        let author = client.keys().public_key().to_string();

        let following_subscription = Filter::new()
            .kind(Kind::ContactList)
            .author(author);

        let following_peers = subscribe_to_relay(&client, following_subscription).await;

        if !following_peers.is_empty() {
            peers_backup.add_follows(&following_peers[0].tags);
        }
    }

    if followers {
        let bech32 = XOnlyPublicKey::from_bech32(&npub_key).expect("Cannot get the XOnlyPubKey");

        let followers_subscription = Filter::new()
            .kind(Kind::ContactList)
            .pubkey(bech32);

        let followers_peers = subscribe_to_relay(&client, followers_subscription).await;

        if !followers_peers.is_empty() {
            peers_backup.add_followers(followers_peers);
        }
    }
    
    println!("Data extracted from the relays so disconnecting...");
    client.disconnect().await.unwrap();
    println!("Client disconnected succesfully");

    peers_backup.export_peers();
}

async fn subscribe_to_relay(client: &Client, subscription: Filter) -> Vec<Event> {

    let mut events: Vec<Event> = vec![];

    println!("Nostr client trying to subscribe to the relay...");
    client.subscribe(vec![subscription]).await;
    println!("Subscribed!");

    let mut notifications = client.notifications();
    while let Ok(notification) = notifications.recv().await {
        // Process Events 
        if let RelayPoolNotification::Event(_url, event) = notification {
            // Populate events until we receive a empty Message
            events.push(event);   
        }
        // Process Messages
        else if let RelayPoolNotification::Message(_url, message) = notification {
            if message::RelayMessage::Empty == message {
                println!("The relays does not have more data to offer, unsubscribe...");
                break;
            }
        }
    }
    // Unsubscribe from the applied filters
    client.unsubscribe().await;
    println!("Unsubscribed!");

    events
}