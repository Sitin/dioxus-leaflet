# Dioxus Leaflet

[Leaflet](https://leafletjs.com/) bindings for [Dioxus](https://dioxuslabs.com).

For now, this project is in POC state. It seems that it's possible to provide a meaningful functionality
for most of the users without covering the entire Leaflet API.

I'll would be happy to get any help from those who interested in a quick map solution for Dioxus.

## Example

```rust
use dioxus::prelide::*;
use dioxus_leaflet::{Map, TileLayer};

#[component]
pub fn MapContainer() -> Element {
    rsx! {
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
                opacity: 0.5,
            }
        }
    }
}
```

## Development

The library is located in the [dioxus-leaflet](dioxus-leaflet) folder.

### Example App

Run the following command in the root of your project to start [example](./example) app for default platform:

```bash
dx serve -p example
```

To run for a different platform, use the `--platform platform` flag. E.g.

```bash
dx serve -p example --platform desktop
```
