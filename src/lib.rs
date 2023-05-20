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
        plugin::{AnimationPlugin, AudioPlugin, BluetoothPlugin, GuiPlugin, ZoetropePlugins},
        setup::{cleanup_menu, setup_menu, Resolutions, RunningStates, Settings},
    };
}
