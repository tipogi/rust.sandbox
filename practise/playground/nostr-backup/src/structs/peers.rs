use std::fs::File;
use nostr_sdk::{Tag, Event, prelude::ToBech32};
use serde::{Serialize, Deserialize};
use crate::{constants, utils::key_converter, enums::Prefix};

#[derive(Serialize, Deserialize, Debug)]
pub struct Peers {
    following: Vec<String>,
    followers: Vec<String>,
    relays: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relays {
    read: bool,
    write: bool,
    url: String
}

impl Peers {
    pub fn new() -> Self {
        Self {
            following: vec![],
            followers: vec![],
            relays: String::from("")
        }
    }

    pub fn get_followers(&self) -> &Vec<String> {
        &self.followers
    }

    pub fn get_following(&self) -> &Vec<String> {
        &self.following
    }

    pub fn get_relays(&self) -> &String {
        &self.relays
    }
    
    // We will receive all the following npub(s) in hex format
    pub fn add_follows(&mut self, tags: &Vec<Tag>) {
        for pub_key in tags.into_iter() {
            let bech32_pub_key = key_converter::encode_to_bech32(Prefix::Npub, &pub_key.as_vec()[1]);
            self.following.push(bech32_pub_key);
        }
    }

    pub fn add_followers(&mut self, following: Vec<Event>) {
        for event in following.into_iter() {
            let bech32_pub_key = event.pubkey.to_bech32().unwrap();
            self.followers.push(bech32_pub_key);
        }
    }

    pub fn add_relays(&mut self, relays: &String) {
        self.relays = relays.to_string();
    }

    pub fn export_peers(&self) {
        let peers_json = serde_json::to_string(&self).unwrap();
        File::create(constants::BACKUP_FILE).unwrap();
        std::fs::write(constants::BACKUP_FILE, peers_json).unwrap();
        println!("📦 Peers backup succesfull in {}", constants::BACKUP_FILE);
    }
}