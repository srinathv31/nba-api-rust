use serde_json::{Value, Error};

use crate::JSON_CONTENT;

pub fn get_json() -> Result<Value, Box<dyn std::error::Error>> {
    let data: Result<Value, Error> = serde_json::from_str(JSON_CONTENT);
    return Ok(data?);
}
