//! Library assets

use dioxus::prelude::*;

/// Leaflet styles (`leaflet.css`)
pub const LEAFLET_CSS: Asset = asset!("/assets/vendors/leaflet.css");
/// Leaflet JavaScript (`leaflet.js`)
pub const LEAFLET_JS: Asset = asset!("/assets/vendors/leaflet.js");

pub(crate) const DIOXUS_LEAFLET_JS: Asset = asset!("/assets/dioxus-leaflet.js");
