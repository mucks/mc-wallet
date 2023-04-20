use std::num::NonZeroU32;

use ring::aead::*;

use anyhow::Result;
use ring::aead::CHACHA20_POLY1305;
use ring::pbkdf2::derive;
use ring::pbkdf2::PBKDF2_HMAC_SHA256;

fn make_key(password: &[u8]) -> LessSafeKey {
    let salt = b"mc-wallet";

    let mut key = [0; 32];

    derive(
        PBKDF2_HMAC_SHA256,
        NonZeroU32::new(100).unwrap(),
        salt,
        password,
        &mut key,
    );

    let unbound_key = UnboundKey::new(&CHACHA20_POLY1305, &key).unwrap();
    LessSafeKey::new(unbound_key)
}

pub fn encrypt(password: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    let nonce = Nonce::assume_unique_for_key([0u8; 12]);
    let in_out = &mut data.to_vec();

    let key = make_key(password);

    key.seal_in_place_append_tag(nonce, Aad::empty(), in_out)
        .map_err(|err| anyhow::anyhow!("could not encrypt data: {err}"))?;

    Ok(in_out.clone())
}

pub fn decrypt(password: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    let nonce = Nonce::assume_unique_for_key([0u8; 12]);
    let key = make_key(password);
    let mut data = data.to_vec();
    let data = key
        .open_in_place(nonce, Aad::empty(), &mut data)
        .map_err(|err| anyhow::anyhow!("could not decrypt data: {err}"))?;
    Ok(data.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() -> Result<()> {
        let password = b"password";
        let data = b"hello world";

        let encrypted = encrypt(password, data)?;
        let decrypted = decrypt(password, &encrypted)?;

        assert_eq!(data, &decrypted[..]);
        Ok(())
    }
}
