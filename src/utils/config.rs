use std::env;
use dotenvy::dotenv;

pub trait Config {
    fn load_env();
}

pub struct AppConfig;

impl Config for AppConfig {
    fn load_env() {
        dotenv().ok();
    }
}

impl AppConfig {
    pub fn check_vars(str_vec: Vec<&str>) -> bool {
        let mut result = true;
        for var in str_vec {
            if env::var(var).is_err() {
                result = false;
                break;
            }
        }
        result
    }

    pub fn get_var(var: &str) -> String {
        env::var(var).unwrap()
    }
}