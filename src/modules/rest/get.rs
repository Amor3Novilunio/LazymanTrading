use crate::{modules::env, std_error_exit};
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

    // Request Builder
    let request = match client
        .get(props.url)
        .query(&props.params)
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
