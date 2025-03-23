use dioxus::document::EvalError;
use dioxus::prelude::*;

use crate::assets::{DIOXUS_LEAFLET_JS, LEAFLET_CSS, LEAFLET_JS};
use crate::utils::{await_js_is_ready, unique_id, IdentifiedElementSpecs, CTX_VAR};

const MAP_ID_PREFIX: &str = "leaflet-map";
const DEFAULT_LAT: f64 = 51.505;
const DEFAULT_LNG: f64 = -0.09;
const DEFAULT_ZOOM: u16 = 13;

// #[derive(PartialEq, Clone, Props)]
// pub struct MapProps {
//     class: Option<String>,
//     style: Option<String>,
//     lat: Option<f64>,
//     lng: Option<f64>,
//     zoom: Option<u16>,
//     children: Element,
// }

#[derive(Clone)]
pub(crate) struct MapContainerSpecs(IdentifiedElementSpecs);

impl MapContainerSpecs {
    pub fn new<T: AsRef<str>>(id: T) -> Self {
        Self(IdentifiedElementSpecs::new(id))
    }

    pub fn id(&self) -> &str {
        self.0.id()
    }
}

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

    let container_specs = MapContainerSpecs::new(id.clone());

    use_context_provider(|| Signal::new(container_specs.clone()));

    let initiated = use_resource(move || {
        let container_specs = container_specs.clone();

        async move {
            if await_js_is_ready().await.is_none() {
                return Err(EvalError::Unsupported);
            }

            let id = container_specs.id().to_owned();

            let init_js = format!(r#"
                let map = window.L.map('{id}').setView([{lat}, {lng}], {zoom});
                {CTX_VAR}.addMap('{id}', map);
            "#);

            document::eval(init_js.as_str()).await
        }
    });

    rsx! {
        document::Link { href: LEAFLET_CSS, rel: "stylesheet" }
        document::Script { src: LEAFLET_JS }
        document::Script { src: DIOXUS_LEAFLET_JS }

        div {
            id: "{id}",
            class: class,
            style: style,

            match initiated.value().as_ref() {
                Some(v) => rsx!{
                    match v.as_ref() {
                        Ok(_) => rsx!( ),
                        Err(err) => rsx!( p { "{err:?}" }),
                    }
                    { children },
                },
                _ => rsx!( p { "loading map..." } ),
            }
        }
    }
}
