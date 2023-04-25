use clap::Parser;
use nostr_backup::{args::Args};
use nostr_backup::services::backup;

#[tokio::main]
async fn main() {

   match Args::parse() {
    args if args.backup => {
      let Args { followers, following, key, nsec, .. } = args;
      backup::create_backup_file(followers, following, &key, nsec).await
    }
    _ => println!("No Action!")
  }
}
