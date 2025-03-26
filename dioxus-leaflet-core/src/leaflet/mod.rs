use serde::{Deserialize, Serialize};
use specta::Type;

#[cfg_attr(feature = "generator", derive(Type))]
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Default)]
pub struct MapView {
    pub lat: f64,
    pub lng: f64,
    pub zoom: u16,
}

#[cfg_attr(feature = "generator", derive(Type))]
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct TileLayer {
    pub tiles: String,
    pub options: TileLayerOptions,
}

#[cfg_attr(feature = "generator", derive(Type))]
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Default)]
pub struct TileLayerOptions {
    #[serde(rename = "maxZoom")]
    pub max_zoom: Option<u16>,
    pub attribution: Option<String>,
    pub opacity: Option<f32>,
}
