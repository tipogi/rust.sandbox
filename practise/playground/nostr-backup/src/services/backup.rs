use nostr_sdk::{Keys, prelude::*};
use crate::utils::relay;
use crate::enums::Prefix;
use crate::utils::key_converter;
use crate::structs::peers::Peers;


pub async fn create_backup_file(wrapped_key: Option<String>, followers: bool, following: bool) {

    let key = wrapped_key.unwrap();
    
    // Before make the backup, check if the bech32 and HEX key are correct ones
    // Also if our key is in HEX format, encode to bech32 format
    let (npub_key, _hex_key) = key_converter::display_key_info(&key, Prefix::Npub);

    let nostr_key = Keys::from_pk_str(&npub_key).unwrap();
    let client = Client::new(&nostr_key);
    
    relay::add_to_client(&client).await;

    println!("🟪 The client connecting to relays...");
    client.connect().await;
    println!("✅ Connected!");

    let mut peers_backup = Peers::new();

    println!("\n📡 Waiting to add a subscription...");

    if following {
        let author = client.keys().public_key().to_string();

        let following_subscription = Filter::new()
            .kind(Kind::ContactList)
            // is it necessary? it just persist one event and overwrites the actual one?
            .limit(1)
            .author(author);

        let following_peers = subscribe_to_relay(&client, following_subscription).await;

        if !following_peers.is_empty() {
            peers_backup.add_follows(&following_peers[0].tags);
            peers_backup.add_relays(&following_peers[0].content);
            println!("{:?}", peers_backup);
        }
    }

    println!("\n📡 Waiting to add a subscription...");

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
    
    println!("🌅 Data extracted from the relays so disconnecting...");
    client.disconnect().await.unwrap();
    println!("✅ Client disconnected succesfully");

    peers_backup.export_peers();
}

async fn subscribe_to_relay(client: &Client, subscription: Filter) -> Vec<Event> {

    let mut events: Vec<Event> = vec![];

    println!("📣 Adding a filter to the subscription pipe...");
    client.subscribe(vec![subscription]).await;
    println!("✅ Subscribed!");

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
                println!("🪦 The relays does not have more data to offer, unsubscribe...");
                break;
            }
        }
    }
    // Unsubscribe from the applied filters
    client.unsubscribe().await;
    println!("✅ Unsubscribed!");

    events
}