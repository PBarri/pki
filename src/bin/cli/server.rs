use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct ServerCommand {
    /// The CA name to initialize.
    #[structopt(long, short)]
    pub ca_name: Option<String>,

    /// The key size in bytes. Can be either 2048 or 4096.
    #[structopt(long, short = "s")]
    pub key_size: Option<u16>,
}