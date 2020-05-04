use structopt::StructOpt;
use cli::arguments::*;
use ::pki::*;

mod cli;

fn main() {
    let context = Arguments::from_args();

    execute(context);
}

fn execute(context: Arguments) {
    println!("cli options: {:?}", context);

    match context.cmd {
        Subcommands::INIT(init) => init_command_execution(init),
        Subcommands::CLIENT(client) => client_command_execution(client),
        Subcommands::SERVER(server) => server_command_execution(server),
        Subcommands::CA(ca) => ca_command_execution(ca)
    }
}

fn init_command_execution(cmd: InitCommand) {
    println!("Command options: {:?}", cmd);

    let pki = cmd.pki.unwrap_or("testPki".to_string());
    let ca_name = cmd.ca_name.unwrap_or("defaultCa".to_string());
    let key_size: u16 = cmd.key_size.unwrap_or(4096);

    pki::ca::create_ca(pki.as_str(), ca_name.as_str(), key_size).expect("There was an error creating the CA");
}

fn client_command_execution(cmd: CertificateCommand) {
    println!("Command options: {:?}", cmd);

    let pki = cmd.pki.unwrap_or("testPki".to_string());
    let ca_name = cmd.ca_name.unwrap_or("defaultCa".to_string());
    let cert_name = cmd.name.expect("The name of the certificate must be provided");
    // let key_size: u16 = cmd.key_size.unwrap_or(4096);

    pki::client::create_client_certificate(pki.as_str(), ca_name.as_str(), cert_name.as_str()).expect("There was a problem creating the client certificate");    
}

fn server_command_execution(cmd: CertificateCommand) {
    println!("Command options: {:?}", cmd);

    let pki = cmd.pki.unwrap_or("testPki".to_string());
    let ca_name = cmd.ca_name.unwrap_or("defaultCa".to_string());
    let cert_name = cmd.name.expect("The name of the certificate must be provided");
    // let key_size: u16 = cmd.key_size.unwrap_or(4096);

    pki::server::create_server_certificate(pki.as_str(), ca_name.as_str(), cert_name.as_str()).expect("There was a problem creating the server certificate");
}

fn ca_command_execution(cmd: CaCommand) {
    println!("Command options: {:?}", cmd);
    pki::ca::create_ca("testPki", "defaultCa", 4096).expect("There was an error creating the CA");
}