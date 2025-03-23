use std::time::Duration;

use async_std::task::sleep;
use dioxus::prelude::*;

pub(crate) const CTX_VAR: &str = "window._dioxusLeaflet";

const RETRY_INTERVAL: Duration = Duration::from_millis(200);
const RETRY_ATTEMPTS: usize = 20;

pub(crate) async fn await_js_is_ready() -> Option<()> {
    await_leaflet_is_ready().await?;
    await_dioxus_leaflet_is_ready().await?;
    Some(())
}

async fn await_leaflet_is_ready() -> Option<()> {
    for _ in 0..RETRY_ATTEMPTS {
        if document::eval(r#"return window.L.version;"#).await.is_ok() {
            return Some(());
        }
        sleep(RETRY_INTERVAL).await;
    }
    None
}

async fn await_dioxus_leaflet_is_ready() -> Option<()> {
    for _ in 0..RETRY_ATTEMPTS {
        if document::eval(format!(r#"return {CTX_VAR}.version;"#).as_str()).await.is_ok() {
            return Some(());
        }
        sleep(RETRY_INTERVAL).await;
    }
    None
}