use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SpecResponseItem {
    pub item: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct SpecResponse {
    pub name: SpecResponseItem,
    pub processor: SpecResponseItem,
    pub ram: SpecResponseItem,
    pub storage: SpecResponseItem,
}