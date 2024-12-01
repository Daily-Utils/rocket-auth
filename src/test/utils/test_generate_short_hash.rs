#[cfg(test)]
mod test {
    use crate::utils::generate_short_hash::{decrypt, encrypt};

    #[test]
    fn test_encrypt_decrypt() {
        let key = "12345678901234567890123456789012";
        let data = "Hello, World!";
        let encrypted_data = encrypt(data, key, 16);
        let decrypted_data = decrypt(&encrypted_data, key).unwrap();
        assert_eq!(data, decrypted_data);
    }

    #[test]
    #[should_panic(expected = "decryption failed")]
    fn test_encrypt_decrypt_with_different_key() {
        let key = "12345678901234567890123456789012";
        let data = "Hello, World!";
        let encrypted_data = encrypt(data, key, 16);
        let key = "12345678901234567890123456789013";
        let decrypted_data = decrypt(&encrypted_data, key).expect("decryption failed");
        assert_ne!(data, decrypted_data);
    }
}
