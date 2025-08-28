use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    pub origin_price: f64,     // base origin price
    pub sell_percentage: f64,  // e.g. 20.0 = +20%
    pub buy_price: f64,         // when the percentage is reached this is how much you buy
    pub buy_percentage: f64,   // e.g. -20.0 = -20%
    pub limiter: f64,          // max USDT spend
}
