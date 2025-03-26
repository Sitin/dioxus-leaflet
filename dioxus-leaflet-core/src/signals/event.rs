use serde::{Deserialize, Serialize};
use specta::Type;

#[cfg_attr(feature = "generator", derive(Type))]
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub enum ApiEvent {
    Debug(String),
}