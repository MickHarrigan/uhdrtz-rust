pub use nokhwa;
mod audio;
mod bluetooth;
mod camera;
mod gui;
mod plugin;
mod setup;
mod zoetrope;

pub mod prelude {
    pub use crate::{
        // temporary
        bluetooth::{async_converter_arduino_finder, async_converter_arduino_reader},
        camera::{hash_available_cameras, VideoFrame, VideoStream},
        gui::{
            gui_camera_control, gui_change_mask, gui_full, gui_open, gui_set_crosshair,
            CameraCrosshair, CameraCrosshairTag, CameraMaskSetting, CameraMaskTag, ColorSettings,
            UiState,
        },
        plugin::{
            AnimationPlugin, AudioPlugin, BluetoothPlugin, CameraPlugin, GuiPlugin, ZoetropePlugins,
        },
        setup::{cleanup_menu, setup_menu, Resolutions, RunningStates, Settings},
    };
}
