pub async fn create_backup_file(_followers: bool, _following: bool, key: &String, private_key: bool) {
    key_converter::decode_to_hex(key);
    // TODO: Cannot let to add the private key. Set clap
    // Create the keys
    let nostr_key = match private_key {
        true => Keys::from_sk_str(key).unwrap(),
        _ => Keys::from_pk_str(key).unwrap()
    };

    let client = Client::new(&nostr_key);

    println!("KEYS: {:?}", client.keys());

    for relay in relay::get_relays().iter() {
        println!("Adding relay {} to the client...", relay);
        client.add_relay(relay, None).await.unwrap();
    }

    let timeout = Duration::from_secs(10);

    let list = client.get_contact_list(Some(timeout)).await.unwrap();

    println!("{:?}", list);

    //client.connect().await;
    
    // Get the following peers
    // Get the followers peers
    // Create the JSON file
}