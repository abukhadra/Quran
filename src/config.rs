use dotenv::dotenv;
use std::env;

pub fn get_config( key: &str) -> String {
    dotenv().ok();
    env::var(key)
        .expect("error reading {} from .env")

}