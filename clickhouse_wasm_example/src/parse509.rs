use std::collections::HashMap;
use x509_parser::parse_x509_certificate;
use x509_parser::pem::parse_x509_pem;

use clickhouse_wasm_sdk::clickhouse_fatalf;
use clickhouse_wasm_udf_bindgen::clickhouse_udf;

#[clickhouse_udf]
pub fn parse_pem(data: String) -> HashMap<String, String> {
    let (_, pem) = match parse_x509_pem(data.as_bytes()) {
        Ok(p) => p,
        Err(e) => clickhouse_fatalf!("Error parsing PEM: {:?}", e),
    };
    let res_x509 = match parse_x509_certificate(&pem.contents) {
        Ok((_, res)) => res,
        Err(e) => clickhouse_fatalf!("Error parsing X509 certificate: {:?}", e),
    };
    let cert = &res_x509.tbs_certificate;

    HashMap::from([
        (String::from("version"), cert.version.0.to_string()),
        (String::from("issuer"), cert.issuer.to_string()),
        (String::from("subject"), cert.subject.to_string()),
        (
            String::from("not_before"),
            cert.validity.not_before.timestamp().to_string(),
        ),
        (
            String::from("not_after"),
            cert.validity.not_after.timestamp().to_string(),
        ),
    ])
}
