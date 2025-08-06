use bech32::{segwit, Hrp};
use serde_bytes::ByteBuf;

use serde::Deserialize;
use std::collections::HashMap;
use x509_parser::parse_x509_certificate;
use x509_parser::pem::parse_x509_pem;

use clickhouse_wasm_sdk::{clickhouse_fatalf, clickhouse_logf};
use clickhouse_wasm_udf_bindgen::clickhouse_udf;


#[clickhouse_udf]
pub fn bech32_encode_udf(prefix: String, data: ByteBuf) -> String {
    let hrp = Hrp::parse(prefix.as_str());
    if hrp.is_err() {
        clickhouse_fatalf!("Invalid HRP: {}", hrp.err().unwrap());
    }
    let hrp = hrp.unwrap();

    let taproot_address = segwit::encode(hrp, segwit::VERSION_1, data.as_ref());
    if taproot_address.is_err() {
        clickhouse_fatalf!("Failed to encode Bech32: {}", taproot_address.err().unwrap());
    }
    return taproot_address.unwrap();
}

#[clickhouse_udf]
pub fn bech32_decode_udf(data:String) -> (String, ByteBuf) {
    let res = segwit::decode(data.as_str());
    if res.is_err() {
        clickhouse_fatalf!("Failed to decode Bech32: {}", res.err().unwrap());
    }
    let (hrp, _version, data) = res.unwrap();
    let hrp = hrp.to_string();
    let data = ByteBuf::from(data);
    return (hrp, data);
}
