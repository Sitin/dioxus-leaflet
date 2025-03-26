use dioxus::document::{Eval, EvalError};
use dioxus::logger::tracing::{error, warn};
use dioxus::prelude::*;

use dioxus_leaflet_core::signals::{ApiEvent, ApiRecv, CtrlRecv, CtrlResp};

use crate::js_bindings::utils::await_js_is_ready;

pub(crate) struct DioxusLeafletInterop {
    main: Eval,
    events: Eval,
}

pub(crate) struct DioxusLeafletSender {
    inner: Eval,
}

pub(crate) struct DioxusLeafletReceiver {
    inner: Eval,
}

impl DioxusLeafletInterop {
    pub async fn create(id: &str) -> Result<Self, EvalError> {
        let (main, events) = create_js_api(id).await?;
        Ok(Self { main, events })
    }

    #[inline(always)]
    pub fn send_ctrl(&self, data: CtrlRecv) -> Result<(), EvalError> {
        self.main.send(data)
    }

    #[inline(always)]
    pub async fn recv_ctrl(&mut self) -> Result<CtrlResp, EvalError> {
        self.events.recv().await
    }

    pub fn split(self) -> (DioxusLeafletSender, DioxusLeafletReceiver) {
        (
            DioxusLeafletSender { inner: self.main },
            DioxusLeafletReceiver { inner: self.events },
        )
    }
}

impl DioxusLeafletSender {
    #[inline(always)]
    pub fn send<T: Into<ApiRecv>>(&self, data: T) -> Result<(), EvalError> {
        self.inner.send(data.into())
    }

    #[inline(always)]
    pub fn send_ctrl(&self, data: CtrlRecv) -> Result<(), EvalError> {
        self.inner.send(data)
    }
}

impl DioxusLeafletReceiver {
    #[inline(always)]
    pub async fn recv(&mut self) -> Result<ApiEvent, EvalError> {
        self.inner.recv().await
    }

    #[inline(always)]
    pub async fn recv_ctrl(&mut self) -> Result<CtrlResp, EvalError> {
        self.inner.recv().await
    }

    pub async fn expect_ctrl(&mut self, value: CtrlResp) -> Result<(), EvalError> {
        let res = self.recv_ctrl().await?;
        if res != value {
            return Err(EvalError::Communication(
                format!("Unexpected response while awaiting for being ready: {:?}", res)
            ));
        }
        Ok(())
    }
}

async fn create_js_api(id: &str) -> Result<(Eval, Eval), EvalError> {
    await_js_is_ready().await?;

    let mut js_main_handler = create_js_main_api(id).await?;
    let js_events_handler = create_js_events_api(id, &mut js_main_handler).await?;

    Ok((js_main_handler, js_events_handler))
}

async fn create_js_main_api(id: &str) -> Result<Eval, EvalError> {
    let mut js_main_handler = {
        let js_code = format!(r#"
            await window.__dioxusLeaflet.init(dioxus, "{id}");
        "#);

        document::eval(js_code.as_str())
    };

    loop {
        match js_main_handler.recv::<CtrlResp>().await {
            Ok(CtrlResp::Created) => break,
            Ok(res) => {
                warn!("unexpected response during main handler creation: {:?}", res);
            }
            Err(err) => {
                error!("error during initialization: {:?}", err);
                return Err(err);
            }
        }
    }

    Ok(js_main_handler)
}

async fn create_js_events_api(id: &str, js_main_api: &mut Eval) -> Result<Eval, EvalError> {
    let mut recv_api = {
        let js_code = format!(r#"
            await window.__dioxusLeaflet.registerEventsChannel(dioxus, "{id}");
        "#);

        document::eval(js_code.as_str())
    };

    if let Err(err) = js_main_api.send(CtrlRecv::Init) {
        error!("unable to send init request: {:?}", err);
        return Err(err);
    }

    for handler in [&mut recv_api, js_main_api] {
        match handler.recv().await {
            Ok(CtrlResp::Initialized) => {}
            Ok(res) => {
                return Err(EvalError::Communication(format!("unexpected response during initialization: {:?}", res)));
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    Ok(recv_api)
}
