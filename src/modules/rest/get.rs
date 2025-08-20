use crate::{modules::env::load::load_env, std_error_exit};
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
    url: String,
    params: Vec<(String, String)>,
}

pub async fn get(props: GetProps) {
    let client = Client::new();
    let env = load_env();

    let api_key = match env.get("api_key") {
        Some(res)=>res,
        None => std_error_exit!(format!("Failed To Deserialize | ERR: {err}")),
    };

    // Req
    let request = match client.get(props.url).query(&props.params).header("X-MBX-APIKEY", env.get("api_key")).send().await {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Failed To Get Request | ERR: {err}")),
    };

    // Res
    match request.json::<HttpBinResponse>().await {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("Failed To Get Response | ERR: {err}")),
    };
}
