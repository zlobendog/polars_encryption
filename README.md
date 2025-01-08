# Polars Encryption

This is a plugin to extend Polars with encryption algorithm **AES-GSM-SIV** written in Rust for a blazing fast (c) performance.

# Changelog
08-01-2025: updated for latest versions of Polars and rust


# Sources

This plugin is built using a guide by [Marco Gorelli](https://marcogorelli.github.io/polars-plugins-tutorial/).

The encryption algorithm crate is [this](https://docs.rs/aes-gcm-siv/latest/aes_gcm_siv/). The specific implementation is a 256bit version of it. 

## Usage

There are two methods: encrypt and decrypt. Both require key and nonce, as per algorithm specification.

```python
import polars as pl
from polars_encryption import encrypt, decrypt

# Define the DataFrame
df = pl.DataFrame({
    "plaintext": ["Hello, world!", "Polars is fast!", "Encrypt me!"]
})

# Define the encryption key and nonce
key = b"an example very very secret key."
nonce = b"unique nonce"  # 12 bytes (96 bits)

# Encrypt the plaintext column
df_encrypted = df.with_columns(
    encrypt(pl.col("plaintext"), key=key, nonce=nonce).alias("ciphertext")).with_columns(decrypt(pl.col("ciphertext"), key=key, nonce=nonce).alias("decrypted"))

print((df["plaintext"] == df_encrypted["decrypted"]).all())

print(df_encrypted)
```
