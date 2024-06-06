import polars as pl
from polars_encryption import encrypt, decrypt


def test_validity_of_decryption():
    df = pl.DataFrame({
        "plaintext": ["Hello, world!", "Polars is fast!", "Encrypt me!"]
    })

    # Define the encryption key and nonce
    key = b"an example very very secret key."
    nonce = b"unique nonce"  # 12 bytes (96 bits)

    # Encrypt the plaintext column
    df_encrypted = df.with_columns(
    encrypt(pl.col("plaintext"), key=key, nonce=nonce).alias("ciphertext")).with_columns(decrypt(pl.col("ciphertext"), key=key, nonce=nonce).alias("decrypted"))


    assert (df["plaintext"] == df_encrypted["decrypted"]).all(), "Decrypted values do not match the original plaintext values."
