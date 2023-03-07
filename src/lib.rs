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
        gui::{
            gui_camera_control, gui_change_mask, gui_full, gui_open, gui_set_crosshair,
            CameraCrosshair, CameraCrosshairTag, CameraMaskSetting, CameraMaskTag, CameraMovement,
            UiState,
        },
        plugin::ZoetropePlugins,
    }; // temporary names for right now, these may change based on the current needs of the project
}
