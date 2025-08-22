use crate::{modules::env, runtime::functions::get_unix_clock, std_error_exit};
use reqwest::{Client, StatusCode};

// Props
pub struct GetProps {
    pub url: String,
    pub params: Option<Vec<(String, String)>>,
}

// Send Request
pub async fn request(props: GetProps) -> (StatusCode, serde_json::Value) {
    let client = Client::new();
    let env = env::load_env();

    let signature: Vec<(String, String)> = vec![("signature".to_string(), env.secret_key)];

    let timestamp: Vec<(String, String)> = vec![("timestamp".to_string(), get_unix_clock())];

    let recv_window: Vec<(String, String)> = vec![("recvWindow".to_string(), 10000.to_string())];

    let compiled_query = || {
        let mut compiled: Vec<(String, String)> = Vec::new();
        compiled.extend(signature);
        compiled.extend(timestamp);
        compiled.extend(recv_window);
        compiled
    };

    let query: Vec<(String, String)> = match props.params {
        Some(mut params) => {
            params.extend(compiled_query());
            params
        }
        None => compiled_query(),
    };

    // Request Builder
    let request = match client
        .get(props.url)
        .query(&query)
        .header("X-MBX-APIKEY", env.api_key.to_string())
        .send()
        .await
    {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Failed To Get Request | ERR: {err}")),
    };

    // Get the Status
    let status = request.status();

    // Get the body
    let body = match request.text().await {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Failed To Read Body | ERR: {err}")),
    };

    // body Parsing
    let result = match serde_json::from_str::<serde_json::Value>(&body) {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Failed To Parse JSON | ERR: {err} | raw={body}")),
    };

    // Return
    (status, result)
}
