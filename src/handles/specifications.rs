use crate::models::specifications::{SpecResponse, SpecResponseItem};

use anyhow::Result;
use axum::{extract::Path, http::StatusCode, response::{IntoResponse, Json}};
use serde::Deserialize;
use sysinfo::{System, RefreshKind, Disks, Networks, Components};

#[derive(Debug, Deserialize)]
pub struct PathPayload {
    pub item: String,
}

pub async fn get_spec(Path(path_payload): Path<PathPayload>) -> impl IntoResponse {
    let mut sys = System::new_all();
    let mut item = SpecResponseItem{item: path_payload.item.clone(), description: String::new()};
    sys.refresh_all();

    // ToDo: システム情報を取得
    // match path_payload.item.as_str() {
    //     "name" => item.description.insert()
    // }
}