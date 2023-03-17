pub use nokhwa;
mod audio;
mod bluetooth;
mod camera;
mod gui;
mod plugin;
mod states;
mod zoetrope;

pub mod prelude {
    pub use crate::{
        camera::{VideoFrame, VideoStream},
        gui::{
            gui_camera_control, gui_change_mask, gui_full, gui_open, gui_set_crosshair,
            CameraCrosshair, CameraCrosshairTag, CameraMaskSetting, CameraMaskTag, ColorSettings,
            UiState,
        },
        plugin::{
            AnimationPlugin, AudioPlugin, BluetoothPlugin, CameraPlugin, GuiPlugin, ZoetropePlugins,
        },
        states::RunningStates,
    };
}
