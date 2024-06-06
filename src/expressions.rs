#![allow(clippy::unused_unit)]
use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use aes_gcm_siv::{aead::{Aead, KeyInit}, Aes256GcmSiv, Nonce};
use serde::Deserialize;
use base64;
use std::fmt::Write;

#[derive(Deserialize)]
pub struct KeyKwargs {
    pub key: Vec<u8>,
    pub nonce: Vec<u8>,
}

#[polars_expr(output_type=String)]
fn encrypt(inputs: &[Series], kwargs: KeyKwargs) -> PolarsResult<Series> {
    let s = &inputs[0];
    let ca = s.str()?;
    let cipher = Aes256GcmSiv::new_from_slice(&kwargs.key).expect("key length should be correct");
    let nonce = Nonce::from_slice(&kwargs.nonce);

    let encrypted_values = ca.apply_to_buffer(|value: &str, output: &mut String| {
        let encrypted_data = cipher
            .encrypt(nonce, value.as_bytes())
            .expect("encryption should not fail");
        let encoded_data = base64::encode(&encrypted_data);
        write!(output, "{}", encoded_data).unwrap();
    });

    Ok(encrypted_values.into_series())
}

#[polars_expr(output_type=String)]
fn decrypt(inputs: &[Series], kwargs: KeyKwargs) -> PolarsResult<Series> {
    let s = &inputs[0];
    let ca = s.str()?;
    let cipher = Aes256GcmSiv::new_from_slice(&kwargs.key).expect("key length should be correct");
    let nonce = Nonce::from_slice(&kwargs.nonce);

    let decrypted_values = ca.apply_to_buffer(|value: &str, output: &mut String| {
        let encrypted_data = base64::decode(value).expect("decoding should not fail");
        let decrypted_data = cipher
            .decrypt(nonce, encrypted_data.as_ref())
            .expect("decryption should not fail");
        let decrypted_string = String::from_utf8(decrypted_data).expect("utf8 conversion should not fail");
        write!(output, "{}", decrypted_string).unwrap();
    });

    Ok(decrypted_values.into_series())
}