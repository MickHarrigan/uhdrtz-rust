pub mod bluetooth;
pub mod camera;
pub mod plugin;
pub use nokhwa;
mod systems;

pub mod prelude {
    pub use crate::{
        camera::{VideoFrame, VideoStream},
        plugin::ZoetropePlugin,
        systems::{UiState,ui_test, open_window},
    }; // temporary names for right now, these may change based on the current needs of the project
}
