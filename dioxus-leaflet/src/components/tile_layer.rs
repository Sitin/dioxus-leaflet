use dioxus::prelude::*;

use dioxus_leaflet_core::leaflet;

use crate::api::LeafletApi;
use crate::utils::unique_id;

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
    let api = use_context::<Signal<LeafletApi>>();

    let id = use_memo(unique_id);
    let tiles = use_memo(move || tiles.clone());
    let attribution = use_memo(move || attribution.clone());

    use_effect(move || {
        api.read().add_tile_layer(id.read().clone(), leaflet::TileLayer {
            tiles: tiles.read().clone(),
            options: leaflet::TileLayerOptions {
                max_zoom: Some(max_zoom),
                opacity: Some(opacity),
                attribution: Some(attribution.read().clone()),
            },
        });
    });

    use_effect(use_reactive!(|opacity| {
        api.read().set_tile_layer_opacity(id.read().clone(), opacity)
    }));

    rsx!()
}
