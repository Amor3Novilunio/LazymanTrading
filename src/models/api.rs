use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Api {
    binance_api: Vec<String>, // Collection of Binance API
    secret_key: String,       // Secret Key
}
