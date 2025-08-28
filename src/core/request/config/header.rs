use reqwest::header::HeaderMap;

pub fn headers(api_key: String) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("X-MBX-APIKEY", api_key.parse().unwrap());
    headers
}
