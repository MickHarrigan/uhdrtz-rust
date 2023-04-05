use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};
use bevy_core_pipeline::clear_color::ClearColor;
use bevy_egui::EguiPlugin;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_kira_audio::prelude::AudioPlugin as KiraAudioPlugin;
use bevy_tokio_tasks::TokioTasksPlugin;

use crate::audio::{audio_modulation_rotation, audio_setup};
use crate::bluetooth::{async_spawner, RotationInterval};
use crate::camera::VideoFrame;
use crate::gui::{
    gui_camera_control, gui_change_mask, gui_full, gui_open, gui_set_crosshair, CameraCrosshair,
    CameraMaskSetting, ColorSettings, UiState,
};
use crate::zoetrope::{
    zoetrope_animation, zoetrope_next_camera_frame, zoetrope_setup, ZoetropeMaxInterval,
};

pub struct ZoetropePlugins; // High level Grouped Plugins for end use
pub struct BluetoothPlugin; // Bluetooth only section
pub struct CameraPlugin; // Physical Camera setup plugin
pub struct GuiPlugin; // Gui controls and setup
pub struct AnimationPlugin; // Plugin for the animation and its controls
pub struct AudioPlugin; // Plugin for playing the music
struct BasePlugin; // Miscellaneous and background things that need to be set for the typical ZoetropePlugins

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .build()
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::BorderlessFullscreen,
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .add_system(bevy::window::close_on_esc);
    }
}

impl Plugin for BluetoothPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TokioTasksPlugin::default())
            .insert_resource(RotationInterval(0))
            .add_startup_system(async_spawner);
    }
}
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VideoFrame(Handle::default()));
    }
}

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ZoetropeMaxInterval(10))
            .add_startup_system(zoetrope_setup)
            .add_system(zoetrope_next_camera_frame)
            .add_system(zoetrope_animation);
    }
}

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .insert_resource(UiState {
                is_window_open: false,
            })
            .init_resource::<CameraMaskSetting>()
            .init_resource::<ColorSettings>()
            .insert_resource(CameraCrosshair(false))
            .add_system(gui_full)
            .add_system(gui_open)
            .add_system(gui_camera_control)
            .add_system(gui_set_crosshair)
            .add_system(gui_change_mask);
    }
}

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(KiraAudioPlugin)
            .add_startup_system(audio_setup)
            .add_system(audio_modulation_rotation);
    }
}

impl PluginGroup for ZoetropePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BasePlugin)
            .add(AudioPlugin)
            .add(AnimationPlugin)
            .add(CameraPlugin)
            .add(GuiPlugin)
            .add(BluetoothPlugin)
    }
}
