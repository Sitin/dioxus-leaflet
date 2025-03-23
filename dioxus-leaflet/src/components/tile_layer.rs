use dioxus::document::EvalError;
use dioxus::prelude::*;

use crate::components::map::MapContainerSpecs;
use crate::utils::{await_js_is_ready, unique_id, IdentifiedElementSpecs, CTX_VAR};

const TILES_LAYER_ID_PREFIX: &str = "leaflet-tile-layer";
const DEFAULT_TILES: &str = r#"https://tile.openstreetmap.org/{z}/{x}/{y}.png"#;
const DEFAULT_ZOOM: u16 = 18;
const DEFAULT_ATTRIBUTION: &str = r#"&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a>"#;
const DEFAULT_OPACITY: f32 = 1.0;

#[component]
pub fn TileLayer(
    #[props(default=DEFAULT_TILES, into)]
    tiles: String,
    #[props(default=DEFAULT_ZOOM)]
    max_zoom: u16,
    #[props(default=DEFAULT_ATTRIBUTION, into)]
    attribution: String,
    #[props(default=DEFAULT_OPACITY)]
    opacity: f32,
) -> Element {
    let container_specs = use_context::<Signal<MapContainerSpecs>>();
    let layer_specs = use_signal(|| IdentifiedElementSpecs::new(format!("{}-{}", TILES_LAYER_ID_PREFIX, unique_id())));

    let tiles = use_memo(move || tiles.clone());
    let attribution = use_memo(move || attribution.clone());

    use_effect(use_reactive!(|opacity| {
        let map_id = container_specs.read().id().to_owned();
        let layer_id = layer_specs.read().id().to_owned();

        spawn(async move {
            let change_opasity_js = format!(r#"
                {CTX_VAR}.getTileLayer('{map_id}', '{layer_id}').setOpacity({opacity});
            "#);

            _ = document::eval(change_opasity_js.as_str()).await;
        });
    }));

    let future = use_resource(move || {
        async move {
            if await_js_is_ready().await.is_none() {
                return Err(EvalError::Unsupported);
            }

            let map_id = container_specs.read().id().to_owned();
            let layer_id = layer_specs.read().id().to_owned();

            let add_layer_js = format!(r#"
                let tileLayer = L.tileLayer(
                    "{tiles}",
                    {{
                        maxZoom: {max_zoom},
                        attribution: '{attribution}',
                        opacity: {opacity},
                    }},
                );
                {CTX_VAR}.addTileLayer('{map_id}', '{layer_id}', tileLayer);
            "#);

            document::eval(add_layer_js.as_str()).await
        }
    });

    match future.value().as_ref() {
        Some(v) => match v.as_ref() {
            Ok(_) => rsx!( ),
            Err(err) => rsx!( p { "{err:?}" }),
        },
        _ => rsx!( p { "loading tiles..." } ),
    }
}
