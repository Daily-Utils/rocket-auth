use aes::Aes256;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use hex::{decode, encode};
use rand::{thread_rng, RngCore};
use std::error::Error;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn encrypt(data: &str, key: &str, iv_size: usize) -> String {
    let key: &[u8] = key.as_bytes();
    let mut iv: Vec<u8> = vec![0u8; iv_size];
    thread_rng().fill_bytes(&mut iv);

    let cipher: Cbc<Aes256, Pkcs7> = Aes256Cbc::new_from_slices(key, &iv).unwrap();

    let encrypted_data = cipher.encrypt_vec(data.as_bytes());

    format!("{}{}", encode(iv), encode(encrypted_data))
}

pub fn decrypt(encrypted_data: &str, key: &str) -> Result<String, Box<dyn Error>> {
    let key: &[u8] = key.as_bytes();

    let (iv_hex, data_hex) = encrypted_data.split_at(32);

    let iv: Vec<u8> = decode(iv_hex)?;
    let data: Vec<u8> = decode(data_hex)?;

    let cipher: Aes256Cbc = Aes256Cbc::new_from_slices(key, &iv)?;

    let decrypted_data: Vec<u8> = cipher.decrypt_vec(&data)?;

    print!("decrypted_data: {:?}", String::from_utf8(decrypted_data.clone())?);

    Ok(String::from_utf8(decrypted_data)?)
}
