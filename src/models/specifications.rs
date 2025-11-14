use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SpecResponseItem {
    pub item: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpecResponse {
    pub name: SpecResponseItem,
    pub processor: SpecResponseItem,
    pub ram: SpecResponseItem,
    pub storage: SpecResponseItem,
}
