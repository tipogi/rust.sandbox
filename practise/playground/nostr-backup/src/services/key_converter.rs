use bech32::{ FromBase32, ToBase32, Variant };
use clap::ValueEnum;

#[derive(Clone, Debug, ValueEnum)]
pub enum Prefix {
    Npub,
    Nsec,
}

pub fn decode_to_hex(bech32_key: &str) {
    let (_, data, _) = bech32::decode(bech32_key).unwrap();
    println!("HEX: {}", hex::encode(Vec::<u8>::from_base32(&data).unwrap()));
}

pub fn encode_to_bech32(hrp: &str, hex_key: &str) -> String {

    let bech32 = bech32::encode(
        // Human Readble Part
        hrp,
        hex::decode(hex_key)
            .map_err(|_e| println!("erro"))
            .unwrap()
            .to_base32(),
        Variant::Bech32,
    ).unwrap();

    println!("BECH32 key: {}", bech32);

    bech32
}

pub fn bech32_checksum(key: &String) -> bool {
    // Check if the key is bech32 or hex format
    match bech32::decode(key) {
        Ok(_) => true,
        Err(_e) => false
    }
}