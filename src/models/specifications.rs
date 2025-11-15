use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SpecResponse {
    pub hostname: String,
    pub os: String,
    pub kernel: String,
    pub processor: String,
    pub ram: String,
}
