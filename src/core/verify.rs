use crate::core::request::wallet::account_status;


pub async fn verify() {
    // verify if the secret key and api key is valid
    account_status().await
}
