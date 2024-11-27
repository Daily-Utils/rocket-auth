#[cfg(test)]
mod test {
    use crate::rocket;
    use rocket::local::blocking::Client;
    use rocket::tokio::runtime::Runtime;

    #[test]
    fn test_client_creation() {
        let rt = Runtime::new().unwrap();
        let rocket_instance = rt.block_on(rocket());

        let _ = Client::tracked(rocket_instance).expect("valid rocket instance");
    }
}
