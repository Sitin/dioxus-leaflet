use serde::{Deserialize, Serialize};
use specta::Type;

#[cfg_attr(feature = "generator", derive(Type))]
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtrlRecv {
    Init,
    Stop,
}

#[cfg_attr(feature = "generator", derive(Type))]
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtrlResp {
    Created,
    Initialized,
    Ready,
    Stopped,
}
