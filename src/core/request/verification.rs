use crate::{
    core::request::{
        config::{header::headers, query::query},
        types::RequestProps,
    },
    modules::rest,
};

pub async fn account_status(props: RequestProps) {
    // Parameters
    let query = Some(query(&props, None));
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
