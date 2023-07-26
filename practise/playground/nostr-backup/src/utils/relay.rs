use nostr_sdk::Client;

const RELAY: [&str; 7] = [
    "wss://relay.damus.io/", 
    "wss://relay.snort.social/",
    "wss://eden.nostr.land",
    "wss://nos.lol",
    "wss://foolay.nostr.moe/", 
    "wss://nostr.mom/",
    "wss://nostr.wine/"
];

pub async fn add_to_client(client: &Client) {
    for relay in RELAY.into_iter() {
        println!("🟣 Added {} relay to the client", relay);
        client.add_relay(relay, None).await.unwrap();
    }
}