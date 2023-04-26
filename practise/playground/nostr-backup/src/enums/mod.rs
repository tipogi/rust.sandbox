use clap::ValueEnum;

#[derive(Clone, Debug, ValueEnum)]
pub enum Prefix {
    Npub,
    Nsec,
}

impl std::fmt::Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Prefix::Npub => write!(f, "npub"),
            Prefix::Nsec => write!(f, "nsec"),
        }
    }
}