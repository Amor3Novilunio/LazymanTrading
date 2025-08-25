use serde::Deserialize;

#[derive(Deserialize)]
pub struct Token {
    pub origin_price: f64,     // base origin price
    pub sell_percentage: f64,      // e.g. 20.0 = +20%
    pub buy_percentage: f64,       // e.g. -20.0 = -20%
    pub buy_origin_price: f64, // updated origin price
    pub limiter: f64,          // max USDT spend
    pub wallet_usdt: f64,      // wallet balance
}
