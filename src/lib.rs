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
        gui::{set_crosshair, change_mask, open_window, ui_test, Crosshair, MaskImage, MaskSetting, UiState},
        plugin::ZoetropePlugin,
    }; // temporary names for right now, these may change based on the current needs of the project
}
