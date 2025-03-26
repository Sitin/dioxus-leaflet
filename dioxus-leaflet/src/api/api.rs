use async_std::sync::Mutex;
use dioxus::document::EvalError;
use dioxus::logger::tracing::{debug, error, info, warn};
use dioxus::prelude::*;
use futures::channel::mpsc::unbounded;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;

use dioxus_leaflet_core::leaflet::{MapView, TileLayer};
use dioxus_leaflet_core::signals::{ApiEvent, ApiRecv, CtrlRecv, CtrlResp, TileLayerApiRecv};

use crate::api::ApiState;
use crate::js_bindings::interop::{DioxusLeafletInterop, DioxusLeafletReceiver};

enum Command {
    Ctrl(CtrlRecv),
    Api(ApiRecv),
}

pub struct LeafletApi {
    id: String,
    state: SyncSignal<ApiState>,
    recv_tx: Arc<Mutex<UnboundedSender<Command>>>,
    events_rx: UnboundedReceiver<ApiEvent>,
}

impl LeafletApi {
    pub fn new(id: &str, state: SyncSignal<ApiState>) -> Self {
        let (recv_tx, recv_rx) = unbounded();
        let (events_tx, events_rx) = unbounded();

        let id = id.to_string();

        Self::spawn(&id, state, recv_rx, events_tx);

        let recv_tx = Arc::new(Mutex::new(recv_tx));

        Self { id, state, recv_tx, events_rx }
    }

    pub fn set_view(&self, view: MapView) {
        self.api_recv(view);
    }

    pub fn add_tile_layer(&self, id: String, layer: TileLayer) {
        self.api_recv(TileLayerApiRecv::add(id, layer));
    }

    pub fn remove_tile_layer(&self, id: String) {
        self.api_recv(TileLayerApiRecv::remove(id));
    }

    pub fn set_tile_layer_opacity(&self, id: String, opacity: f32) {
        self.api_recv(TileLayerApiRecv::set_opacity(id, opacity));
    }

    pub fn api_recv<T: Into<ApiRecv>>(&self, recv: T) {
        let recv_tx = self.recv_tx.clone();
        let recv = recv.into();
        let mut state = self.state;

        spawn(async move {
            if let Err(err) = recv_tx.lock().await.send(Command::Api(recv)).await {
                error!("Unable to send API request: {err}");
                state.set(ApiState::Failed);
            }
        });
    }

    fn spawn(
        id: &str,
        mut state: SyncSignal<ApiState>,
        mut recv_rx: UnboundedReceiver<Command>,
        events_tx: UnboundedSender<ApiEvent>,
    ) -> Task {
        let id = id.to_string();

        spawn(async move {
            let (sender, mut receiver) = match DioxusLeafletInterop::create(&id).await {
                Ok(channel) => channel.split(),
                Err(_) => {
                    state.set(ApiState::Failed);
                    return;
                }
            };

            state.set(ApiState::Initialized);

            // Await ready state
            if let Err(err) = receiver.expect_ctrl(CtrlResp::Ready).await {
                error!("Failed to reach ready state: {:?}", err);
                state.set(ApiState::Failed);
                return;
            }

            // Now we consider API to ready
            state.set(ApiState::Ready);

            info!("API is ready for {id}");

            // Spawn JavaScript events listener
            Self::spawn_events_loop(state, receiver, events_tx);

            // Start main loop that sends messages to JavaScript
            while let Some(command) = recv_rx.next().await {
                match command {
                    Command::Api(req) => {
                        if let Err(err) = sender.send(req) {
                            error!("Sending request to JS bindings failed: {:?}", err);
                            state.set(ApiState::Failed);
                            break;
                        }
                    }
                    Command::Ctrl(CtrlRecv::Stop) => {
                        debug!("Received stop command");
                        state.set(ApiState::Stopped);
                        _ = sender.send_ctrl(CtrlRecv::Stop);
                        break;
                    }
                    Command::Ctrl(_) => {}
                }
            }

            warn!("API bindings loop stopped");
        })
    }

    fn spawn_events_loop(
        mut state: SyncSignal<ApiState>,
        mut receiver: DioxusLeafletReceiver,
        mut events_tx: UnboundedSender<ApiEvent>)
    {
        spawn(async move {
            while let ApiState::Ready = *state.clone().read() {
                match receiver.recv().await {
                    Ok(event) => {
                        info!("Received event from JS: {:?}", event);
                        if let Err(err) = events_tx.send(event).await {
                            error!("Failed to broadcast api event: {:?}", err);
                            state.set(ApiState::Failed);
                            return;
                        }
                    }
                    Err(EvalError::Serialization(err)) => {
                        warn!("Unable to deserialize event from JS bindings: {:?}", err);
                    }
                    Err(err) => {
                        error!("Failed to get event from JS bindings: {:?}", err);
                        state.set(ApiState::Failed);
                        return;
                    }
                }
            }

            debug!("Exiting events loop.");
        });
    }
}