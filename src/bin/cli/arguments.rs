//! Command Line Interface arguments for the PKI generator application.

use structopt::StructOpt;
use super::ca::*;
use super::init::*;

/// PKI Management tool.
/// A command line application to manage minimal local PKI installations for testing.
/// 
/// Author: Pablo Barrientos.
/// 
/// Please report any issues you might find to https://github.com/PBarri/pki
#[derive(StructOpt, Debug)]
#[structopt(name = "pki")]
pub struct Arguments {

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,
    
    #[structopt(subcommand)]
    pub cmd: Subcommands,
}

#[derive(StructOpt, Debug)]
pub enum Subcommands {

    /// Initializes a new PKI installation
    #[structopt(name = "init")]
    INIT(InitCommand),

    /// Operations with the CA
    #[structopt(name = "ca")]
    CA(CaCommand),

}
