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
            camera_control, change_mask, logical_camera_movement, open_window, set_crosshair,
            ui_test, CrossImage, Crosshair, MaskImage, MaskSetting, Movement, UiState,
        },
        plugin::{AnimationPlugin, BluetoothPlugin, CameraPlugin, GuiPlugin, ZoetropePlugins},
    };
}
