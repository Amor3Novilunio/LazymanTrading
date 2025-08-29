use serde_json::Value;

use crate::{
    core::request::config::{header::headers, signed_query},
    modules::{env, rest},
    runtime::functions::{JoinStringProps, join_string},
};

// ---------------------------------------------
// Request a quote for the requested token pairs
// ---------------------------------------------
pub async fn send_quote(
    from_asset: String,
    to_asset: String,
    from_amount: Option<f64>,
    to_amount: Option<f64>,
) -> Value {
    let mut value: Value = Value::Null;

    // Load The Env
    let env = env::load_env();

    // Convert the api collection to array of Strings
    let binance_api_collection = join_string(JoinStringProps {
        split_prefix: ",".to_string(),
        string_collection: env.api_url_general.clone(),
    });

    // Parameters
    let query = Some(signed_query::query(env.secret_key, {
        let mut extensions = vec![];

        extensions.extend(vec![("fromAsset".to_string(), from_asset)]);
        extensions.extend(vec![("toAsset".to_string(), to_asset)]);

        if let Some(from_amount) = from_amount {
            extensions.extend(vec![("fromAmount".to_string(), from_amount.to_string())]);
        }

        if let Some(to_amount) = to_amount {
            extensions.extend(vec![("toAmount".to_string(), to_amount.to_string())]);
        }

        Some(extensions)
    }));

    let headers = headers(env.api_key);

    // Api Collection Response
    for api_url in binance_api_collection {
        // Send Request For Account Verification If Token Is Still Valid
        let (status, body) = rest::post::request(rest::post::PostProps {
            url: format!("{}/sapi/v1/convert/getQuote", api_url),
            query: query.clone(),
            headers: headers.clone(),
            body: None,
        })
        .await;

        // Conditional Result Handling
        if status.is_success() {
            println!("-- Quote Request Sent Successful --");
            value = body;
            break;
        } else {
            value = body;
            println!("Failed to sendQuote : {} | Payload :{:?}", status, value);
            continue;
        }
    }

    value
}

// -------------------------------------
// Accept the offered quote by quote ID.
// -------------------------------------
pub async fn accept_quote(quote_id: String) {
    // Load The Env
    let env = env::load_env();

    // Convert the api collection to array of Strings
    let binance_api_collection = join_string(JoinStringProps {
        split_prefix: ",".to_string(),
        string_collection: env.api_url_general.clone(),
    });

    // Parameters
    let query = Some(signed_query::query(env.secret_key, {
        let mut extensions = vec![];

        extensions.extend(vec![("quoteId".to_string(), quote_id.to_string())]);

        Some(extensions)
    }));

    let headers = headers(env.api_key);

    // Api Collection Response
    for api_url in binance_api_collection {
        // Send Request For Account Verification If Token Is Still Valid
        let (status_code,value) = rest::post::request(rest::post::PostProps {
            url: format!("{}/sapi/v1/convert/acceptQuote", api_url),
            query: query.clone(),
            headers: headers.clone(),
            body: None,
        })
        .await;

        // Conditional Result Handling
        if status_code.is_success() {
            println!("-- Quote Accepted Successful --");
            break;
        } else {
            println!("Failed to getQuote : {} | Payload :{:?}", status_code, value);
            continue;
        }
    }
}
