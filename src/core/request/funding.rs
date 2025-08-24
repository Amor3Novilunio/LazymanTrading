use serde_json::Value;

use crate::{
    core::request::{
        config::{header::headers, query::query},
        types::RequestProps,
    },
    modules::rest,
};

pub async fn funding(props: RequestProps) -> Value {
    let mut value: Value = Value::Null;

    // Parameters
    let query = Some(query(
        &props,
        Some(vec![
            // ----- Empty Get All
            ("asset".to_string(), "".to_string()),
            // ----- Show Asset to Btc Equivalent to the Payload
            ("needBtcValuation".to_string(), "false".to_string()),
        ]),
    ));

    let headers = headers(&props);

    // Api Collection Response
    for api_url in &props.binance_api_collection {
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
            println!("Error apply handling here meh ill probably just panic it since we are on imperative approach");
            continue;
        }
    }

    value
}
