use dioxus::prelude::*;
use dioxus_leaflet::components::TileLayer;
use dioxus_leaflet::Map;
use std::ops::Deref;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { href: MAIN_CSS, rel: "stylesheet" }
        Container {}
    }
}

#[component]
pub fn Container() -> Element {
    let mut opacity = use_signal(|| 0.5f32);

    rsx! {
        section {
            style: "position: relative;",
            div {
                style: "float: right; width: 200px; max-width: 200px; margin: 10px; margin-top: 0px;",
                h3 { "Controls" }
                hr {}
                h4 { "OpenStreetMap" }
                label {
                    "Opacity: {opacity}"
                }
                input {
                    type: "range",
                    min: 0,
                    max: 100,
                    value: opacity * 100f32,
                    style: "width: 100px; height: 20px",
                    onchange: move |event| {
                        let val = event.value().parse::<u32>().unwrap_or(0);
                        opacity.set(val as f32 / 100f32);
                    },
                }
            }
        }
        div {
            Map {
                style: "min-height: 300px; bottom: 0px; height: calc(100vh - 40px);",
                lat: 51.505,
                lng: -0.09,
                zoom: 14,
                TileLayer {
                    tiles: "https://server.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer/tile/{{z}}/{{y}}/{{x}}",
                    attribution: r#"&copy; <a href="https://www.esri.com/">Esri</a> i-cubed, USDA, USGS, AEX, GeoEye, Getmapping, Aerogrid, IGN, IGP, UPR-EGP, and the GIS User Community"#,
                }
                TileLayer {
                    tiles: "https://tile.openstreetmap.org/{{z}}/{{x}}/{{y}}.png",
                    attribution: r#"&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a>"#,
                    opacity: *opacity.read(),
                }
            }
        }
    }
}
