/// Creates a new certificate with the server extensions enabled
pub fn create_server_certificate(pki_name: &str, ca_name: &str, name: &str) -> std::io::Result<()> {          
    // Creates the CSR
    super::create_certificate_request(pki_name, name, super::SERVER_EXTENSION)?;
    
    // Signs the CSR
    super::sign_certificate(pki_name, ca_name, name, super::SERVER_EXTENSION)?;

    Ok(())
}