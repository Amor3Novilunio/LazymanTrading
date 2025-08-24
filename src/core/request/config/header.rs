use reqwest::header::HeaderMap;

use crate::core::request::types::RequestProps;

pub fn headers(props: &RequestProps) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("X-MBX-APIKEY", props.env.api_key.parse().unwrap());
    headers
}
