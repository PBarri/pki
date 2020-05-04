use resource::resource;
use std::process;
use std::fs;
use std::io::Write;

/// Creates a new CA
pub fn create_ca(pki_name: &str, ca_name: &str, key_size: u16) -> std::io::Result<()> {

    /* Creates directory structure:
        - /{pki_name}
        | - /{ca_dir}
        |  | - /certs
        |  | - /private (700)
        |  | - serial [01]
        |  | - index.txt
        | - /{cert_x}
        | - /{cert_y}
    
    This is created with the following commands (unix):
        > mkdir {pki_name} && cd {pki_name}
        > mkdir {ca_dir} && cd {ca_dir}
        > mkdir certs private
        > chmod 700 private
        > echo 01 > serial
        > touch index.txt            

    Command required to create the CA self-signed certificate:
    openssl req -x509 -config openssl.cnf -newkey rsa:2048 -days 365 -out ca_certificate.pem -outform PEM -subj /CN=MyTestCA/ -nodes
    */

    let ca_dir: String = format!("{pki}/{ca}", pki = pki_name, ca = ca_name);
    let private_dir: String = format!("{dir}/private", dir = ca_dir.as_str());

    println!("Creating default CA directory tree at {}!", ca_dir);

    // Creates the necessary folder structure
    fs::create_dir_all(ca_dir.as_str())?;
    fs::create_dir(format!("{dir}/certs", dir = ca_dir))?;
    fs::create_dir(private_dir.as_str())?;

    // Creates the serial file and writes the 01
    let mut serial_file = fs::File::create(format!("{dir}/serial", dir = ca_dir))?;
    serial_file.write_all(b"01")?;

    // Creates the index.txt file
    fs::File::create(format!("{dir}/index.txt", dir = ca_dir))?;

    // Creates default openssl.cnf file.
    // TODO: Change this for default installation urls!
    //let default_conf = resource::resource_str!("resources/openssl.cnf");

    let default_conf = resource!("resources/openssl.cnf"); 

    let mut conf_file = fs::File::create(format!("{dir}/openssl.cnf", dir = ca_dir))?;
    conf_file.write_all(default_conf.as_ref())?;

    // Create the self-signed certificate
    let command = format!("openssl req -x509 -config openssl.cnf -newkey rsa:4096 -days 365 -out ca_certificate.pem -outform PEM -subj /CN={ca_name}/ -nodes",  
        ca_name = ca_name);
    let mut cmd: process::Command = super::execute_command(ca_dir.as_str(), command.as_str())?;
    
    let output = cmd.output().expect("There was an error generating the self-signed certificate for the CA");
    /* Write this to a log
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    */
    Ok(())
}