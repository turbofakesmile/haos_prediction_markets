use std::{
    process::Command,
    sync::{Arc, RwLock},
};

use axum::{http::StatusCode, response::IntoResponse, Extension};
use tracing::info;

use crate::config::ServerConfig;

#[derive(Debug, Clone)]
pub struct RebootService {
    enabled: bool,
    auth_required: bool,
}

impl RebootService {
    pub fn new(config: &ServerConfig) -> Self {
        Self {
            enabled: config.allow_reboot(),
            auth_required: config.requires_auth(),
        }
    }

    async fn initiate_reboot(&self) -> Result<(), String> {
        match Command::new("sudo")
            .args(["shutdown", "-r", "now"])
            .output()
        {
            Ok(_) => {
                info!("Reboot command executed successfully");
                Ok(())
            }
            Err(e) => {
                let error_msg = format!("Failed to execute reboot command: {}", e);
                info!("{}", error_msg);
                Err(error_msg)
            }
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn requires_auth(&self) -> bool {
        self.auth_required
    }
}

pub async fn reboot_handler(
    Extension(service): Extension<Arc<RwLock<RebootService>>>,
) -> impl IntoResponse {
    let service = service.read().unwrap();

    if !service.is_enabled() {
        return (
            StatusCode::FORBIDDEN,
            "Reboot functionality is disabled".to_string(),
        )
            .into_response();
    }

    info!("Server reboot initiated");

    // Clone the service for the async block
    let service_clone = RebootService {
        enabled: service.enabled,
        auth_required: service.auth_required,
    };

    // Drop the read lock before spawning
    drop(service);

    // Spawn the reboot task
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        if let Err(e) = service_clone.initiate_reboot().await {
            info!("Reboot failed: {}", e);
        }
    });

    (StatusCode::OK, "Server reboot initiated".to_string()).into_response()
}
