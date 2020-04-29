use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum CaCommand {
    
    /// Command to create a new CA
    #[structopt()]
    CREATE(CreateCommand),

    /// Revokes a certificate
    #[structopt()]
    REVOKE(RevokeCommand),
}

#[derive(Debug, StructOpt)]
pub struct CreateCommand {

    /// The CA name to initialize.
    #[structopt(long, short)]
    pub ca_name: Option<String>,

    /// The key size in bytes. Can be either 2048 or 4096.
    #[structopt(long, short = "s")]
    pub key_size: Option<u16>,

}

#[derive(Debug, StructOpt)]
pub struct RevokeCommand {
    /// The CA used for revocation.
    pub ca_name: String,

    /// The certificate alias to revoke
    pub certificate_id: String,
}