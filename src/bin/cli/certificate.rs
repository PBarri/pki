use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct CertificateCommand {

    /// The PKI name to use.
    #[structopt(long, short)]
    pub pki: Option<String>,
    
    /// The CA to use.
    #[structopt(long, short)]
    pub ca_name: Option<String>,

    /// The name of the certificate.
    #[structopt(long, short)]
    pub name: Option<String>,

    /// The key size in bytes. Can be either 2048 or 4096.
    #[structopt(long, short = "s")]
    pub key_size: Option<u16>,
}