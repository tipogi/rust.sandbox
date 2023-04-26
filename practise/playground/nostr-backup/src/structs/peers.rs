use std::fs::File;
use nostr_sdk::{Tag, Event, prelude::ToBech32};
use serde::{Serialize, Deserialize};
use crate::{constants, services::key_converter::decode_to_hex};

#[derive(Serialize, Deserialize, Debug)]
pub struct Peers {
    following: Vec<String>,
    followers: Vec<String>
}

impl Peers {
    pub fn new() -> Self {
        Self {
            following: vec![],
            followers: vec![]
        }
    }

    // We will receive all the following npub(s) in hex format
    pub fn add_follows(&mut self, tags: &Vec<Tag>) {
        for pub_key in tags.into_iter() {
            self.following.push(pub_key.as_vec()[1].clone());
        }
    }

    pub fn add_followers(&mut self, following: Vec<Event>) {
        for event in following.into_iter() {
            let bech32_pub_key = event.pubkey.to_bech32().unwrap();
            self.followers.push(decode_to_hex(&bech32_pub_key));
        }
    }

    pub fn export_peers(&self) {
        let peers_json = serde_json::to_string(&self).unwrap();
        File::create(constants::BACKUP_FILE).unwrap();
        std::fs::write(constants::BACKUP_FILE, peers_json).unwrap();
        println!("Peers backup succesfull in {}", constants::BACKUP_FILE);
    }

    pub fn import_peers(&self) {

    }
}