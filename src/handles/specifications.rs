use crate::models::specifications::{SpecResponse, SpecResponseItem};

use anyhow::Result;
use axum::{extract::Path, http::StatusCode, response::{IntoResponse, Json}};
use serde::Deserialize;
use sysinfo::{System};

#[derive(Debug, Deserialize)]
pub struct PathPayload {
    pub item: String,
}

pub async fn get_spec(Path(path): Path<PathPayload>) -> impl IntoResponse {
    let mut sys = System::new_all();
    sys.refresh_all();

    let description = match path.item.as_str() {
        "name" => System::host_name().unwrap_or_else(|| "Unknown".to_string()),
        "os" => System::name().unwrap_or_else(|| "Unknown".to_string()),
        "kernel" => System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
        "cpu" => {
            let cpu = sys.cpus().first().unwrap();
            // ToDo: コア数も返す
            format!("{}", cpu.brand())
        },
        "memory" => format!("{} MB", sys.total_memory() / 1024 / 1024),
        _ => return (StatusCode::BAD_REQUEST, Json(SpecResponseItem {
            item: path.item.clone(),
            description: "Unknown item type".to_string(),
        })).into_response(),
    };

    Json(SpecResponseItem {
        item: path.item,
        description,
    }).into_response()
}
