use std::time::Duration;

use async_std::task::sleep;
use dioxus::document::EvalError;
use dioxus::prelude::*;

pub(crate) const CTX_VAR: &str = "window.__dioxusLeaflet.version";

const RETRY_INTERVAL: Duration = Duration::from_millis(50);
const RETRY_ATTEMPTS: usize = 20;

pub(super) async fn await_js_is_ready() -> Result<(), EvalError> {
    for _ in 0..RETRY_ATTEMPTS {
        if document::eval(format!(r#"return {CTX_VAR};"#).as_str()).await.is_ok() {
            return Ok(());
        }
        sleep(RETRY_INTERVAL).await;
    }
    Err(EvalError::InvalidJs("Missing JS bindings for 'dioxus-leaflet'".to_string()))
}
