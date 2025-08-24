use reqwest::{header::HeaderMap, Client, StatusCode};
use serde_json::{Value};

use crate::std_error_exit;

pub struct PostProps {
    pub url: String,
    pub query: Option<Vec<(String, String)>>,
    pub body: Option<Value>,
    pub headers: HeaderMap,
}

pub async fn request(props: PostProps) -> (StatusCode, serde_json::Value) {
    let client = Client::new();

    // Request Builder
    let request = match client
        .post(&props.url)
        .query(&props.query)
        .headers(props.headers)
        .json(&props.body)
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
