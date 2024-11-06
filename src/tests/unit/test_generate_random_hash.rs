#[cfg(test)]
mod tests {
    use crate::utils::generate_random_hash::generate_random_hash_function;

    #[test]
    fn test_generate_random_hash() {
        let hash = generate_random_hash_function(64);
        assert_eq!(hash.len(), 64);
    }
}