use async_std::task::sleep;
use dioxus::document::{Eval, EvalError};
use dioxus::logger::tracing::{error, info, warn};
use dioxus::prelude::*;
use futures_util::StreamExt;

use dioxus_leaflet_core::leaflet::MapView;
use dioxus_leaflet_core::signals::{ApiRecv, CtrlResp, MapApiRecv};

use crate::api::{ApiState, LeafletApi};
use crate::assets::{BINDINGS_JS, LEAFLET_CSS};
use crate::utils::{unique_id, CTX_VAR};

const MAP_ID_PREFIX: &str = "leaflet-map";
const DEFAULT_LAT: f64 = 51.505;
const DEFAULT_LNG: f64 = -0.09;
const DEFAULT_ZOOM: u16 = 13;

#[component]
pub fn Map(
    #[props(default = "", into)]
    class: String,
    #[props(default = "", into)]
    style: String,
    #[props(default=DEFAULT_LAT)]
    lat: f64,
    #[props(default=DEFAULT_LNG)]
    lng: f64,
    #[props(default=DEFAULT_ZOOM)]
    zoom: u16,
    children: Element,
) -> Element {
    let id = format!("{}-{}", MAP_ID_PREFIX, unique_id());

    let state = use_signal_sync(ApiState::default);
    let api = use_signal(|| LeafletApi::new(&id, state));

    use_context_provider(|| api);

    use_effect(move || {
        match *state.read() {
            ApiState::Created => {}
            ApiState::Initialized => {}
            ApiState::Ready => {
                api.read().set_view(MapView {
                    lat,
                    lng,
                    zoom,
                });
                api.read().api_recv(ApiRecv::Debug("Hello from Dioxus!".into()));
            }
            ApiState::Stopped => {}
            ApiState::Failed => {}
        }
    });

    rsx! {
        document::Link { href: LEAFLET_CSS, rel: "stylesheet" }
        document::Script { src: BINDINGS_JS }

        div {
            id: "{id}",
            class: class,
            style: style,

            match *state.read() {
                ApiState::Created => rsx! {
                    p { "Initializing map..." }
                },
                ApiState::Initialized => rsx! {
                    p { "Loading map..." }
                },
                ApiState::Ready => rsx!{
                    { children },
                },
                ApiState::Failed => rsx!{
                    p { "Failed to load map!" }
                },
                ApiState::Stopped => rsx!( p { "Stopped." } ),
            }
        }
    }
}
