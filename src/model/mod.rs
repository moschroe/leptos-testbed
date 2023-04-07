use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use log::info;
use std::time::{Instant, SystemTime};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfigInfo {
    pub app: String,
    pub configs: Vec<String>,
}

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    _ = FetchAppConfig::register();
}

#[server(FetchAppConfig, "/api")]
pub async fn fetch_apps() -> Result<Vec<AppConfigInfo>, ServerFnError> {
    let elapsed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    dbg!(elapsed, elapsed % 5);
    if elapsed % 5 == 0 {
        error!("fail fetching apps");
        return Err(ServerFnError::ServerError(
            "TESTING FAILURE TESTING FAILURE TESTING FAILURE".into(),
        ));
    }

    info!("fetch_apps()...");
    // let res = reqwest::get("http://10.88.0.1:8080/v1/apps")
    //     .await
    //     .map_err(|err| ServerFnError::ServerError(format!("unable to request apps: {}", err)))?
    //     .json::<Vec<AppConfigInfo>>()
    //     .await
    //     .map_err(|err| ServerFnError::ServerError(format!("unable to decode apps: {}", err)))?;

    // debug shortcut
    let res = vec![
        AppConfigInfo {
            app: "app_a".to_string(),
            configs: vec!["mode_a0".to_string(), "mode_a1".to_string()],
        },
        AppConfigInfo {
            app: "app_b".to_string(),
            configs: vec!["mode_b0".to_string(), "mode_b1".to_string()],
        },
    ];

    Ok(res)
}
