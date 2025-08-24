use crate::modules::env::load::LoadEnv;

pub struct RequestProps {
    pub binance_api_collection: Vec<String>,
    pub env: LoadEnv,
}