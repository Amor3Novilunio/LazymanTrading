use crate::core::request::{types::RequestProps, verification};

pub async fn verify(props: RequestProps) {
    // verify if the secret key and api key is valid
    verification::account_status(RequestProps {
        binance_api_collection: props.binance_api_collection,
        env: props.env,
    })
    .await
}
