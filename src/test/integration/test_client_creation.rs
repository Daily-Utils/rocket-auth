#[cfg(test)]
mod test {
    use crate::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket::tokio::runtime::Runtime;

    #[test]
    fn test_client_creation() {
        let rt = Runtime::new().unwrap();
        let rocket_instance = rt.block_on(rocket());
        let client = Client::tracked(rocket_instance).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Server is running!!!!!");
    }
}
