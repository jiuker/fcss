use serde_derive::*;
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub reg: String,
    pub watch_dir: Vec<String>,
}
