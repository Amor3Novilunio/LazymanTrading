use dotenvy::dotenv;
use std::env;

pub struct LoadEnv {
    pub api_url_general: String,
    pub api_url_spot_trading: String,
    pub api_key: String,
    pub secret_key: String,
}

pub fn load_env() -> LoadEnv {
    LoadEnv {
        api_url_general: api_url_general(),
        api_url_spot_trading: api_url_spot_trading(),
        api_key: api_key(),
        secret_key: secret_key(),
    }
}

pub fn generate_env() {
    // auto create the env
}

pub fn api_url_general() -> String {
    dotenv().ok();
    env::var("API_URL_GENERAL").expect("Env : missing API URL'S General ")
}

pub fn api_url_spot_trading() -> String {
    dotenv().ok();
    env::var("API_URL_SPOT_TRADING").expect("Env : missing API URL'S Spot Trading")
}

pub fn api_key() -> String {
    dotenv().ok();
    env::var("API_KEY").expect("Env Err : missing api_key")
}

pub fn secret_key() -> String {
    dotenv().ok();
    env::var("SECRET_KEY").expect("Env Err : missing secret_key")
}
