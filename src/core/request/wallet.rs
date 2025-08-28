use serde_json::Value;

use crate::{
    core::request::config::{header::headers, signed_query},
    modules::{env, rest},
    runtime::functions::{JoinStringProps, join_string},
};

// ----------------------------
// Fetch account status detail.
// ----------------------------
pub async fn account_status() {
    // Load The Env
    let env = env::load_env();

    // Convert the api collection to array of Strings
    let binance_api_collection = join_string(JoinStringProps {
        split_prefix: ",".to_string(),
        string_collection: env.api_url_general.clone(),
    });

    // Parameters
    let query = Some(signed_query::query(env.secret_key, None));

    let headers = headers(env.api_key);

    // Api Collection Response
    for api_url in binance_api_collection {
        // Send Request For Account Verification If Token Is Still Valid
        let (status, body) = rest::get::request(rest::get::GetProps {
            url: format!("{}/sapi/v1/account/status", api_url),
            query: query.clone(),
            headers: headers.clone(),
        })
        .await;

        // Conditional Result Handling
        if status.is_success() {
            println!("{:?}", status);
            println!("{:?}", body);
            println!("SUCKSESSO");
            break;
        } else {
            println!("{:?}", status);
            println!("{:?}", body);
            println!("Errror");
            continue;
        }
    }
}

// --------------------
// Query Funding Wallet
// --------------------
pub async fn funding_wallet(asset: Option<String>) -> Value {
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

        if let Some(selected_asset) = asset {
            extensions.extend(vec![("asset".to_string(), selected_asset)]);
        }

        extensions.extend(vec![("needBtcValuation".to_string(), "false".to_string())]);

        Some(extensions)
    }));

    let headers = headers(env.api_key);

    // Api Collection Response
    for api_url in binance_api_collection {
        // Send Request For Account Verification If Token Is Still Valid
        let (status, body) = rest::post::request(rest::post::PostProps {
            url: format!("{}/sapi/v1/asset/get-funding-asset", api_url),
            query: query.clone(),
            headers: headers.clone(),
            body: None,
        })
        .await;

        // Conditional Result Handling
        if status.is_success() {
            value = body;
            break;
        } else {
            value = body;
            println!(
                "Error apply handling here meh ill probably just panic it since we are on imperative approach"
            );
            continue;
        }
    }

    value
}
