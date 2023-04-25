pub fn get_relays() -> [String; 6]{
    let relays: [String; 6] = [
        "wss://relay.snort.social/".to_string(),
        "wss://eden.nostr.land".to_string(),
        "wss://nos.lol".to_string(),
        "wss://relay.damus.io/".to_string(), 
        "wss://foolay.nostr.moe/".to_string(), 
        "wss://nostr.mom/".to_string()
    ];

    relays
}

pub const BACKUP_FILE: &str = "backup.json";