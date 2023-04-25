use clap::{arg, Parser, command, ArgGroup};

use crate::services::key_converter;

/// A nostr tool that you can encode/decode the key (npub, nsec) types and
/// also create a backup of all your followers/followed peers
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

#[clap(group(
    //  input error handling: only exactly ONE of 'n' args can be used
    ArgGroup::new("backup_conditions")
        .required(false)
        .args(&["followers", "following"])
))]
// In that case the to_hex is not mandatory because it is a boolean
// If we do not add the arguments means that we do not want encode to hex format
/*#[clap(group(
    //  key command has mandatory arguments as kind and to_hex
    ArgGroup::new("key_conversion")
        .args(&["key"])
        .requires_all(&["kind", "to_hex"])
))]*/

pub struct Args {
    /// boolean flag to create the account backup from a key
   #[arg(
        long, 
        default_value_t = false, 
        required = false,
        requires = "key",
    )]
    pub backup: bool,

    /// boolean flag to create the backup of the followers
    #[arg(
        long, 
        default_value_t = false, 
        requires = "backup",
    )]
    pub followers: bool,

    /// boolean flag to create the backup of the following peers
    #[arg(
        long, 
        default_value_t = false, 
        requires = "backup",
    )]
    pub following: bool,

    /// if the key is in HEX format, we do not know if the key is private or public. The default option is npub.
    /// We do NOT recommend to use the private key to do the backup, better use the public key (npub).
    #[arg(
        long, 
        default_value_t = false, 
    )]
    pub nsec: bool,

    /// the key that you want to use with the command (either hex or bech32). In case you are going to do the backup,
    /// use your public key not the private one
   #[arg(
        long,
        value_name = "KEY"
   )]
   pub key: String,

   /// the kind of entity (npub/nsec/note) being converted from hex to bech32-formatted string
   #[arg(
        long,
        value_name = "KIND",
        requires = "key",
    )]
    pub kind: Option<key_converter::Prefix>,

    /// boolean flag indicating to convert keys from bech32 to hex. If not it would be in opposite way
    #[arg(
        long,
        default_value_t = false, 
    )]
    pub to_hex: bool,
}