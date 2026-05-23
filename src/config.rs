use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[allow(dead_code)]
pub struct Config {
    pub rpc_url: Option<String>,
    pub default_account: Option<String>,
}
