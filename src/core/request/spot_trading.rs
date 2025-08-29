use reqwest::header::HeaderMap;
use serde_json::Value;

use crate::{
    modules::{env, rest},
    runtime::functions::{JoinStringProps, join_string},
};

// -------------------------------------
// Latest price for a symbol or symbols.
// -------------------------------------
// query_value = XRPUSDT, BTCUSDT
pub async fn market_price_ticker(query_value: String) -> Value {
    let mut value = Value::Null;

    // Load The Env
    let env = env::load_env();

    // Convert the api collection to array of Strings
    let binance_api_collection = join_string(JoinStringProps {
        split_prefix: ",".to_string(),
        string_collection: env.api_url_spot_trading.clone(),
    });

    // Query
    let query = {
        let mut compiled_query: Vec<(String, String)> = Vec::new();

        let to_array = join_string(JoinStringProps {
            split_prefix: ",".to_string(),
            string_collection: query_value,
        });

        let to_string = serde_json::to_string(&to_array).unwrap();

        compiled_query.extend(vec![("symbols".to_string(), to_string)]);

        Some(compiled_query)
    };

    // Api Collection Response
    for api_url in binance_api_collection {
        // Send Request For Account Verification If Token Is Still Valid
        let (status, body) = rest::get::request(rest::get::GetProps {
            url: format!("{}/api/v3/ticker/price", api_url),
            query: query.clone(),
            headers: HeaderMap::new(),
        })
        .await;

        // Conditional Result Handling
        if status.is_success() {
            value = body;
            break;
        } else {
            value = body;
            println!("Failed to Ticker/Price : {} | Payload :{:?}", status, value);
            continue;
        }
    }
    value
}
