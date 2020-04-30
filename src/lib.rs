pub mod pki {

    use std::process;

    // TODO: Add logging
    // TODO: Check and handle errors!

    const SERVER_EXTENSION: &'static str = "server";
    const CLIENT_EXTENSION: &'static str = "client";

    pub mod ca {

        use std::fs;
        use std::io::prelude::*;
        use std::process;

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

            // Create the self-signed certificate
            let command = format!("openssl req -x509 -config openssl.cnf -newkey rsa:{} -days 365 -out {}/ca_certificate.pem -outform PEM -subj /CN={}/ -nodes", 
                key_size, 
                ca_dir.as_str(), 
                ca_name);
            let mut cmd: process::Command = super::execute_command(command.as_str());
            cmd.output().expect("There was an error generating the self-signed certificate for the CA");

            Ok(())
        }

        
    }

    pub mod server {

        /// Creates 
        pub fn create_server_certificate() {            
            super::sign_certificate("cert", super::SERVER_EXTENSION);
            println!("Creating a new server certificate");
        }

    }

    pub mod client {

        pub fn create_client_certificate() {
            super::sign_certificate("cert", super::CLIENT_EXTENSION);
            println!("Creating a new client certificate");
        }

    }

    pub mod revocation {

        pub fn revoke_certificate() {
            println!("Revoking the certificate");
        }

    }

    pub mod certificate {

        /// Exports the certificate in #PKCS12 format
        pub fn export_certificate(certificate: String, password: &str) {
            /* Command to execute:
                > openssl pkcs12 -export -out {certificate}.p12 -in {certificate}/client_certificate.pem -inkey {certificate}/private_key.pem \
                    -passout pass:{password}
            */
            println!("Certificate exported!");
        }

    }

    // Private functions that needs to be accessed from the pki submodules

    /// Signs a certificate request of a given certificate
    fn sign_certificate(certificate: &str, ext: &str) {
        /* Signs a valid csr with the command:
            > openssl ca -config {config_file} -in {req_dir} -out {certificate_dir} -notext -batch -extensions {extensions}
        */
        println!("Certificate signed!");
    }

    /// Creates a new key and a CSR associated to that key
    fn create_certificate_request(name: String, ext: String) {
        /* Checks that the name does not exists.
        
           Generate the directory for the name:
            > mkdir {name} && cd {name}
            > openssl genrsa -out private_key.pem {key_size}
            > openssl req -new -key private_key.pem -out req.pem -outform PEM -subj /CN=$({name})/O={extension}/ -nodes

        */
        println!("CSR created!");
    }

    /// Function that handles the OS abstraction and creates the required shell. Returns a std::process::Command
    fn execute_command(command: &str) -> process::Command {
        println!("Executing command!");
        let shell = if cfg!(unix) {
            "sh"
        } else if cfg!(windows) {
            "cmd"
        } else {
            "unknown"
        };

        let mut cli = process::Command::new(shell);
        cli.arg(command);

        return cli;
    }
}