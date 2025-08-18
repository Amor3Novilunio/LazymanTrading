use rust_decimal::Decimal;
use serde::Deserialize;

pub enum ConfigOptions {
    Token(Api),
    Api(Token),
}

// --------------------
// Token Type
// --------------------
#[derive(Deserialize)]
pub struct Token {
    origin_price: Decimal,     // base origin price
    sell_percentage: f64,      // e.g. 20.0 = +20%
    buy_percentage: f64,       // e.g. -20.0 = -20%
    buy_origin_price: Decimal, // updated origin price
    limiter: Decimal,          // max USDT spend
    wallet_usdt: Decimal,      // wallet balance
}

// --------------------
// Api Type
// --------------------
#[derive(Deserialize)]
pub struct Api {
    binance_api: Vec<String>, // Collection of Binance API
    secret_key: String,       // Secret Key
}
