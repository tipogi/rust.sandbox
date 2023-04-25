use dialoguer::Input;
use nostr_sdk::Result;

const ENTER_NPUB: &str = "To make the backup of the account, you need to add the public key";
const ENTER_RELAY_INDEX: &str = "\nTo connect to a subscription feed, we need a relay. Choose from the list one";

pub fn get_public_key() -> Result<String> {
    let npub: String = Input::new()
        .with_prompt(ENTER_NPUB)
        .interact_text()?;
    Ok(npub)
}

pub fn get_relay_index() -> Result<u8> {
    let index: u8 = Input::new()
        .with_prompt(ENTER_RELAY_INDEX)
        .interact_text()?;
    Ok(index)
}