use std::str::FromStr;

use dialoguer::Input;
use nostr_sdk::{EventBuilder, Kind, Tag, Keys};
use nostr_sdk::prelude::*;
use crate::enums::Prefix;
use crate::structs::peers::Peers;
use crate::utils::{key_converter, relay};
use crate::constants;

pub async fn create_new(wrapped_key: Option<String>) {

    let key = wrapped_key.unwrap();
    
    let (nsec_key, hex_key) = key_converter::display_key_info(&key, Prefix::Nsec);

    // Create new client and add the relays URL to contact them
    let client = Client::new(&Keys::from_sk_str(&nsec_key).unwrap());

    relay::add_to_client(&client).await;
    
    let json_file = match std::fs::read_to_string(constants::BACKUP_FILE) {
        Ok(file) => file,
        Err(e) => {
            println!("❌ Could not find the backup file. Create the backup and after clone the account\n");
            panic!("ERROR: {:?}", e)
        }
    };
    let backup = serde_json::from_str::<Peers>(&json_file).unwrap();

    println!("🟪 The client connecting to relays...");
    client.connect().await;
    println!("✅ Connected to the relays!");
    
    if !backup.get_following().is_empty() {
        let mut tags: Vec<Tag> = vec![];

        let following_peers = backup.get_following().into_iter();

        // Create a list of the following peers
        for peer_pub_key in following_peers  {
            let tag = Tag::ContactList { 
                pk: XOnlyPublicKey:: from_bech32(peer_pub_key).unwrap(),
                relay_url: None, 
                alias: None
            };
            tags.push(tag);
        }

        // Create the event to follow nostr peers
        let event_builder = EventBuilder::new(
            // Event Kind: 3, Contants - NIP02
            Kind::ContactList,
            // TODO: We should add here the relays that we trust
            backup.get_relays(),
            &tags
        );
        // Get the key pairs to sign the event
        let keys = Keys::new(SecretKey::from_str(&hex_key).unwrap());
        let contact_list_event = EventBuilder::to_event(event_builder, &keys).unwrap();

        println!("📜 Following the nostr peers of the backup...");
        let res = client.send_event(contact_list_event).await;
        println!("👀 The event to follow the peers broadcasted with eventId of: {:?}", res.unwrap());
    }

    if !backup.get_followers().is_empty() {
        let message = Input::<String>::new().with_prompt("📝 What message do you want to send your nostr followers").interact_text().unwrap();
    
        println!("📫 Sending all the followers a DM to notify that you have a new account...");
        for peer_pub_key in backup.get_followers().into_iter() {
            client.send_direct_msg(
                XOnlyPublicKey::from_bech32(peer_pub_key).unwrap(), 
                &message)
            .await.unwrap();
            println!("📨 the follower with the {:?} key, has a message in its inbox", peer_pub_key);
        }
    }   

    println!("🌅 Data extracted from the relays so disconnecting...");
    client.disconnect().await.unwrap();
    println!("✅ Client disconnected succesfully");
}

#[cfg(test)]
mod tests {
    
}