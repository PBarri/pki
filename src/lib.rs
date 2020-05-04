pub mod pki {

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

    pub mod ca {

        use std::fs;
        use std::io::prelude::*;
        use std::process;
        use resource::resource;

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

        
    }

    pub mod server {

        /// Creates 
        pub fn create_server_certificate(pki_name: &str, ca_name: &str, name: &str) -> std::io::Result<()> {          
            // Creates the CSR
            super::create_certificate_request(pki_name, name, super::SERVER_EXTENSION)?;
            
            // Signs the CSR
            super::sign_certificate(pki_name, ca_name, name, super::SERVER_EXTENSION)?;

            Ok(())
        }

    }

    pub mod client {

        pub fn create_client_certificate(pki_name: &str, ca_name: &str, name: &str) -> std::io::Result<()> {

            // Creates the CSR
            super::create_certificate_request(pki_name, name, super::CLIENT_EXTENSION)?;

            super::sign_certificate(pki_name, ca_name, name, super::CLIENT_EXTENSION)?;

            Ok(())
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
}