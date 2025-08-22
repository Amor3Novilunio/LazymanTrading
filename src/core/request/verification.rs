use reqwest::header::HeaderMap;

use crate::{
    modules::{env::load::LoadEnv, hmac::sha256, rest},
    runtime::functions::{QueryStringProps, get_unix_clock, query_string},
};

pub struct AccountStatusProps {
    pub binance_api_collection: Vec<String>,
    pub env: LoadEnv,
}

fn query(props: &AccountStatusProps) -> Vec<(String, String)> {
    // Query
    let timestamp: Vec<(String, String)> = vec![("timestamp".to_string(), get_unix_clock())];
    let recv_window: Vec<(String, String)> = vec![("recvWindow".to_string(), 10000.to_string())];

    // Signature Message
    let message = query_string(QueryStringProps {
        query: {
            let mut compiled: Vec<(String, String)> = Vec::new();
            compiled.extend(timestamp.clone());
            compiled.extend(recv_window.clone());
            compiled
        },
    });

    // Hmac Signature
    let signed_signature = sha256::encrypt(sha256::EncryptProps {
        secret: &props.env.secret_key,
        message: &message,
    });

    // Signature Query
    let signature: Vec<(String, String)> = vec![("signature".to_string(), signed_signature)];

    // final Query Compilation
    let mut compiled: Vec<(String, String)> = Vec::new();
    compiled.extend(timestamp);
    compiled.extend(recv_window);
    compiled.extend(signature);
    compiled
}

fn headers(props: &AccountStatusProps) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("X-MBX-APIKEY", props.env.api_key.parse().unwrap());
    headers
}

pub async fn account_status(props: AccountStatusProps) {
    // Header

    let query = Some(query(&props));
    let headers = headers(&props);

    // Api Collection Response
    for api_url in &props.binance_api_collection {
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
