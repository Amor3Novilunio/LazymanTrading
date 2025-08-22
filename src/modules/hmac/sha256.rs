use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::std_error_exit;

pub struct EncryptProps<'a> {
    pub secret: &'a str,
    pub message: &'a str,
}

type HmacSha256 = Hmac<Sha256>;

pub fn encrypt(props: EncryptProps) -> String {
    // Create HMAC with secret
    let mut hmac = match HmacSha256::new_from_slice(props.secret.as_bytes()) {
        Ok(res) => res,
        Err(err) => std_error_exit!(format!("HmacSha256 Failed to slice secret | ERR: {err} ")),
    };

    // Input the message
    hmac.update(props.message.as_bytes());

    // Finalize and convert to hex string
    let result = hmac.finalize().into_bytes();
    hex::encode(result)
}
