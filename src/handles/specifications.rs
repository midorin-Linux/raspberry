use crate::models::specifications::SpecResponse;

use anyhow::Result;
use axum::{extract::Path, http::StatusCode, response::{IntoResponse, Json}};
use serde::Deserialize;
use sysinfo::{System};

pub async fn get_spec() -> impl IntoResponse {
    let mut sys = System::new_all();
    sys.refresh_all();

    let (hostname, os, kernel, processor_brand, processor_len, ram) = {
        let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());
        let os = System::long_os_version().unwrap_or_else(|| "Unknown".to_string());
        let kernel = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
        let processor_brand = sys.cpus().first().unwrap().brand();
        let processor_len = sys.cpus().len();
        let ram = sys.total_memory() / 1000 / 1000 / 1000;

        (hostname, os, kernel, processor_brand, processor_len, ram)
    };

    let device_spec = SpecResponse {
        hostname: hostname,
        os: os,
        kernel: kernel,
        processor: format!("{} x {}", processor_brand, processor_len),
        ram: format!("{} GB", ram),
    };

    Json(device_spec)
}
