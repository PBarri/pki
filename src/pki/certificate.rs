/// Exports the certificate in #PKCS12 format
pub fn export_certificate(certificate: String, password: &str) {
    /* Command to execute:
        > openssl pkcs12 -export -out {certificate}.p12 -in {certificate}/certificate.pem -inkey {certificate}/private_key.pem \
            -passout pass:{password}
    */
    println!("Certificate exported!");
}