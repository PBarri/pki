pub mod ca;
pub mod client;
pub mod server;
pub mod certificate;

// pub use self::utils;
use std::process;
use std::fs;

// TODO: Add logging
// TODO: Check and handle errors!

struct Extension<'a> {
    group: &'a str,
    extension_name: &'a str,
}

const SERVER_EXTENSION: Extension<'static> = Extension {
    group: "server",
    extension_name: "server_ca_extensions",
};

const CLIENT_EXTENSION: Extension<'static> = Extension {
    group: "client",
    extension_name: "client_ca_extensions",
};

// Private functions that needs to be accessed from the pki submodules

/// Signs a certificate request of a given certificate
fn sign_certificate(pki_name: &str, ca_name: &str, certificate: &str, ext: Extension) -> std::io::Result<()> {

    /* Signs a valid csr with the command:
        > openssl ca -config {config_file} -in {req_dir} -out {certificate_dir} -notext -batch -extensions {extensions}
    */

    let ca_dir = format!("{pki}/{ca}", pki = pki_name, ca = ca_name);
    let certificate_dir = format!("../{dir}", dir = certificate);

    let command = format!("openssl ca -config openssl.cnf -in {cert_dir}/req.pem -out {cert_dir}/certificate.pem -notext -batch -extensions {extensions}",
            cert_dir = certificate_dir.as_str(),
            extensions = ext.extension_name);
    let mut cmd: process::Command = execute_command(ca_dir.as_str(), command.as_str())?;
    let output = cmd.output().expect("There was an error generating the CSR for the certificate");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    println!("Certificate signed!");

    Ok(())
}

/// Creates a new key and a CSR associated to that key
fn create_certificate_request(pki_name: &str, name: &str, ext: Extension) -> std::io::Result<()> {
    /* Checks that the name does not exists.
    
        Generate the directory for the name:
        > mkdir {name} && cd {name}
        > openssl genrsa -out private_key.pem {key_size}
        > openssl req -new -key private_key.pem -out req.pem -outform PEM -subj /CN=$({name})/O={extension}/ -nodes

    */

    // Creates the directory for the certificate
    let cert_dir: String = format!("{pki}/{cert}", pki = pki_name, cert = name);
    fs::create_dir(cert_dir.as_str())?;

    // Create the self-signed certificate
    let generate_privkey_cmdstr = "openssl genrsa -out private_key.pem 4096";
    let mut generate_privkey_cmd: process::Command = execute_command(cert_dir.as_str(), generate_privkey_cmdstr)?;
    let generate_privkey_output = generate_privkey_cmd.output().expect("There was an error generating the private key for the certificate");

    let generate_csr_cmdstr = format!("openssl req -new -key private_key.pem -out req.pem -outform PEM -subj /CN={name}/O={extension}/ -nodes",  
            name = name,
            extension = ext.group);
    let mut generate_csr_cmd: process::Command = execute_command(cert_dir.as_str(), generate_csr_cmdstr.as_str())?;
    let generate_csr_output = generate_csr_cmd.output().expect("There was an error generating the CSR for the certificate");

    Ok(())
}

/// Function that handles the OS abstraction and creates the required shell. Returns a std::process::Command
fn execute_command(path: &str, command: &str) -> Result<process::Command, std::io::Error> {

    let absolute_path = fs::canonicalize(path)?;
    let abs_path_str = absolute_path.to_str().expect("The path is not valid");


    println!("Executing command: {}/>{}", abs_path_str, command);
    let mut cli: process::Command;

    if cfg!(target_os = "windows") {
        cli = process::Command::new("powershell");
    } else {
        cli = process::Command::new("sh");
        cli.arg("-c");
    };

    cli.current_dir(abs_path_str);
    cli.arg(command);

    return Ok(cli);
}