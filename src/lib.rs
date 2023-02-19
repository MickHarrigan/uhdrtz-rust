pub mod audio;
pub mod bluetooth;
pub mod camera;
pub mod gui;
pub mod plugin;
pub use nokhwa;
mod zoetrope;

pub mod prelude {
    pub use crate::{
        camera::{VideoFrame, VideoStream},
        gui::{open_window, ui_test, UiState},
        plugin::ZoetropePlugin,
    }; // temporary names for right now, these may change based on the current needs of the project
}
