use crate::{modules::env, std_error_exit};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct HttpBinResponse {
    pub args: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub origin: String,
    pub url: String,
}

pub struct GetProps {
    pub url: String,
    pub params: Option<Vec<(String, String)>>,
}

pub async fn request(props: GetProps) {
    let client = Client::new();
    let env = env::load_env();

    // Req
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

    // Res
    match request.json().await {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Failed To Get Response | ERR: {err}")),
    }
}
