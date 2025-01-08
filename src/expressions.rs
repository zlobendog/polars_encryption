#![allow(clippy::unused_unit)]
use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;
use base64::{engine::general_purpose, Engine as _};
use std::borrow::Cow;
use aes_gcm_siv::{aead::{Aead, KeyInit}, Aes256GcmSiv, Nonce};

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

    let encrypted_values = ca.apply(|value: Option<&str>| {
        value.map(|v| {
            let encrypted_data = cipher
                .encrypt(nonce, v.as_bytes())
                .expect("encryption should not fail");
            Cow::Owned(general_purpose::STANDARD.encode(encrypted_data)) // Wrap in Cow::Owned
        })
    });

    Ok(encrypted_values.into_series())
}

#[polars_expr(output_type = String)]
fn decrypt(inputs: &[Series], kwargs: KeyKwargs) -> PolarsResult<Series> {
    let s = &inputs[0];
    let ca = s.str()?;

    let cipher = Aes256GcmSiv::new_from_slice(&kwargs.key)
        .map_err(|_| PolarsError::ComputeError("Invalid key length".into()))?;

    let nonce = Nonce::from_slice(&kwargs.nonce);

    let decrypted_values = ca.apply(|value: Option<&str>| {
        value.map(|v| {
            let encrypted_data = general_purpose::STANDARD.decode(v)
                .expect("base64 decoding should not fail");
            let decrypted_data = cipher
                .decrypt(nonce, encrypted_data.as_ref())
                .expect("decryption should not fail");
            let decrypted_string = String::from_utf8(decrypted_data)
                .expect("utf8 conversion should not fail");
            Cow::Owned(decrypted_string)
        })
    });
    
    Ok(decrypted_values.into_series())
}