use serde_derive::*;
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    reg: String,
}
