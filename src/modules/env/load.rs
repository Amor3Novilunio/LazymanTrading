use dotenvy::dotenv;
use std::env;

pub struct LoadEnv {
    pub binance_api: String,
    pub api_key: String,
    pub secret_key: String,
}

pub fn load_env() -> LoadEnv {
    LoadEnv {
        binance_api: binance_api(),
        api_key: api_key(),
        secret_key: secret_key(),
    }
}

pub fn binance_api() -> String {
    dotenv().ok();
    env::var("BINANCE_API").expect("Env Err : missing binance_api")
}

pub fn api_key() -> String {
    dotenv().ok();
    env::var("BINANCE_API_KEY").expect("Env Err : missing api_key")
}

pub fn secret_key() -> String {
    dotenv().ok();
    env::var("BINANCE_SECRET_KEY").expect("Env Err : missing secret_key")
}
