use serde::{Deserialize, Serialize};
use specta::Type;

use crate::leaflet::{MapView, TileLayer};

#[cfg_attr(feature = "generator", derive(Type))]
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub enum ApiRecv {
    Map(MapApiRecv),
    TileLayer { id: String, recv: TileLayerApiRecv },
    Debug(String),
}

#[cfg_attr(feature = "generator", derive(Type))]
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub enum MapApiRecv {
    SetView(MapView),
}

#[cfg_attr(feature = "generator", derive(Type))]
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub enum TileLayerApiRecv {
    Add(TileLayer),
    SetOpacity(f32),
    Remove,
}

impl MapApiRecv {
    pub fn set_view(view: MapView) -> ApiRecv {
        ApiRecv::Map(MapApiRecv::SetView(view))
    }
}

impl TileLayerApiRecv {
    pub fn add(id: String, layer: TileLayer) -> ApiRecv {
        ApiRecv::TileLayer { id, recv: TileLayerApiRecv::Add(layer) }
    }

    pub fn set_opacity(id: String, value: f32) -> ApiRecv {
        ApiRecv::TileLayer { id, recv: TileLayerApiRecv::SetOpacity(value) }
    }

    pub fn remove(id: String) -> ApiRecv {
        ApiRecv::TileLayer { id, recv: TileLayerApiRecv::Remove }
    }
}

impl From<MapView> for ApiRecv {
    fn from(value: MapView) -> Self {
        Self::Map(MapApiRecv::SetView(value))
    }
}
