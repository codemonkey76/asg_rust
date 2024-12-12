use std::env;

use base64::decode;

use crate::error::AppResult;

pub fn get_app_key() -> AppResult<Vec<u8>> {
    let raw_key = env::var("APP_KEY").expect("APP_KEY must be set");
    if raw_key.starts_with("base64:") {
        let base64_key = &raw_key[7..];
        decode(base64_key)
    } else {
        Ok(raw_key.into_bytes())
    }
}
