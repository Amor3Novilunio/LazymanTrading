use crate::{
    modules::hmac::sha256,
    runtime::functions::{get_unix_clock, query_string},
};

pub fn query(
    secret_key: String,
    query_extends: Option<Vec<(String, String)>>,
) -> Vec<(String, String)> {
    // Query
    let timestamp: Vec<(String, String)> = vec![("timestamp".to_string(), get_unix_clock())];
    let recv_window: Vec<(String, String)> = vec![("recvWindow".to_string(), 10000.to_string())];

    // Signature Message
    let message = query_string({
        let mut compiled: Vec<(String, String)> = Vec::new();
        if let Some(queries) = query_extends.clone() {
            compiled.extend(queries);
        };
        compiled.extend(timestamp.clone());
        compiled.extend(recv_window.clone());
        compiled
    });

    // Hmac Signature
    let signed_signature = sha256::encrypt(sha256::EncryptProps {
        secret: &secret_key,
        message: &message,
    });

    // Signature Query
    let signature: Vec<(String, String)> = vec![("signature".to_string(), signed_signature)];

    // final Query Compilation
    let mut compiled: Vec<(String, String)> = Vec::new();
    if let Some(queries) = query_extends {
        compiled.extend(queries);
    };
    compiled.extend(timestamp);
    compiled.extend(recv_window);
    compiled.extend(signature);
    compiled
}
