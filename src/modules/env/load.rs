use dotenvy::dotenv;
use std::collections::HashMap;
use std::env;

pub fn load_env() -> HashMap<String, String> {
    dotenv().ok();
    let mut env = HashMap::new();

    let api_key = env::var("BINANCE_API_KEY").expect("Missing BINANCE_API_KEY");
    env.insert("api_key".to_string(), api_key);

    let secret_key = env::var("BINANCE_SECRET_KEY").expect("Missing BINANCE_SECRET_KEY");
    env.insert("secret_key".to_string(), secret_key);

    env
}
