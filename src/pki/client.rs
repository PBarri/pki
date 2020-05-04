/// Creates a certificate with the client extensions enabled
pub fn create_client_certificate(pki_name: &str, ca_name: &str, name: &str) -> std::io::Result<()> {

    // Creates the CSR
    super::create_certificate_request(pki_name, name, super::CLIENT_EXTENSION)?;

    // Signs the certificate with the requested PKI and CA
    super::sign_certificate(pki_name, ca_name, name, super::CLIENT_EXTENSION)?;

    Ok(())
}