#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ApiState {
    #[default]
    Created,
    Initialized,
    Ready,
    Stopped,
    Failed,
}
