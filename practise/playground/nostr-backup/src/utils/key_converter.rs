use bech32::{ FromBase32, ToBase32, Variant };
use crate::enums::Prefix;



pub fn decode_to_hex(bech32_key: &str) -> String {
    let (_, data, _) = bech32::decode(bech32_key).unwrap();
    let hex_format = hex::encode(Vec::<u8>::from_base32(&data).unwrap());
    hex_format
}

pub fn encode_to_bech32(hrp: Prefix, hex_key: &str) -> String {

    let bech32 = bech32::encode(
        // Human Readble Part
        &hrp.to_string(),
        hex::decode(hex_key)
            .map_err(|_e| println!("erro"))
            .unwrap()
            .to_base32(),
        Variant::Bech32,
    ).unwrap();

    bech32
}

pub fn bech32_checksum(key: &String) -> bool {
    // Check if the key is bech32 or hex format
    match bech32::decode(key) {
        Ok(_) => true,
        Err(_e) => false
    }
}

pub fn display_key_info(key: &String, prefix: Prefix) -> (String, String) {
    let nostr_key_type = bech32_checksum(key);
    let mut bech32 = key.to_string();
    let mut hex = key.to_string();
    if nostr_key_type {
        hex = decode_to_hex(key);
    } else {
        bech32 = encode_to_bech32(prefix, &hex);
    }
    println!("\n############### YOUR KEY INFO ###############");
    println!("🔑 hex format key: {:?}", hex);
    println!("🔑 bech32 format key: {:?}\n", bech32);

    (bech32, hex)
}