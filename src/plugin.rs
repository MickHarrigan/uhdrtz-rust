use bevy::app::PluginGroupBuilder;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
// use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_kira_audio::prelude::AudioPlugin as KiraAudioPlugin;
use bevy_tokio_tasks::TokioTasksPlugin;

use crate::audio::{audio_modulation_rotation, audio_setup, change_audio_volume, Song, Volume};
use crate::bluetooth::{
    async_converter_arduino_finder, async_converter_arduino_reader, ArduinoConnected,
    RotationInterval,
};
use crate::camera::VideoFrame;
use crate::gui::{
    gui_camera_control, gui_full, gui_open, gui_set_crosshair, CameraCrosshair, CameraMaskSetting,
    ColorSettings, UiState,
};
use crate::setup::{cleanup_menu, setup_menu, Resolutions, RunningStates, Settings};
use crate::zoetrope::{
    zoetrope_animation, zoetrope_next_camera_frame, zoetrope_setup, Counter, ZoetropeMaxInterval,
};

pub struct ZoetropePlugins; // High level Grouped Plugins for end use
pub struct BluetoothPlugin; // Bluetooth only section
pub struct CameraPlugin; // Physical Camera setup plugin
pub struct GuiPlugin; // Gui controls and setup
pub struct AnimationPlugin; // Plugin for the animation and its controls
pub struct AudioPlugin; // Plugin for playing the music
struct BasePlugin; // Miscellaneous and background things that need to be set for the typical ZoetropePlugins
struct SetupPlugin; // Things that run within the setup window before the actual Zoetrope things

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        // this is where all the setup things should be converted to be used in main
        app.add_plugins(
            DefaultPlugins
                .build()
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "UHDRTZ Setup".to_string(),
                        position: WindowPosition::Centered {
                            0: MonitorSelection::Primary,
                        },
                        ..default()
                    }),
                    ..default()
                }),
        )
        // frame rate logging of the whole system
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(TokioTasksPlugin::default())
        .add_plugin(EguiPlugin)
        .insert_resource(Resolutions::default())
        .insert_resource(Settings {
            camera: nokhwa::utils::CameraIndex::default(),
            resolution: nokhwa::utils::Resolution::default(),
            frame_rate: 0,
            arduino_connection: false,
            song: None,
        })
        .insert_resource(ArduinoConnected(false))
        .insert_resource(RotationInterval(0))
        .add_state::<RunningStates>()
        .add_system(setup_menu.in_set(OnUpdate(RunningStates::Setup)))
        .add_system(async_converter_arduino_finder.in_schedule(OnEnter(RunningStates::Setup)))
        .add_system(cleanup_menu.in_schedule(OnExit(RunningStates::Setup)));
    }
}

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_system(bevy::window::close_on_esc);
    }
}

impl Plugin for BluetoothPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RotationInterval(0)).add_system(
            async_converter_arduino_reader.in_schedule(OnEnter(RunningStates::Running)),
        );
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
            .insert_resource(Counter(0))
            .add_system(zoetrope_setup.in_schedule(OnEnter(RunningStates::Running)))
            .add_system(zoetrope_next_camera_frame.in_set(OnUpdate(RunningStates::Running)))
            .add_system(zoetrope_animation.in_set(OnUpdate(RunningStates::Running)));
    }
}

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiState {
            is_window_open: false,
        })
        .insert_resource(CameraMaskSetting::default())
        .insert_resource(ColorSettings::default())
        .insert_resource(CameraCrosshair(false))
        .add_system(gui_full.in_set(OnUpdate(RunningStates::Running)))
        .add_system(gui_open.in_set(OnUpdate(RunningStates::Running)))
        .add_system(gui_camera_control.in_set(OnUpdate(RunningStates::Running)))
        .add_system(gui_set_crosshair.in_set(OnUpdate(RunningStates::Running)));
        // .add_system(gui_change_mask.in_set(OnUpdate(RunningStates::Running)));
    }
}

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(KiraAudioPlugin)
            .insert_resource(Song("None".to_string()))
            .insert_resource(Volume(0.5))
            .add_system(audio_setup.in_schedule(OnEnter(RunningStates::Running)))
            .add_system(audio_modulation_rotation.in_set(OnUpdate(RunningStates::Running)))
            .add_system(change_audio_volume.in_set(OnUpdate(RunningStates::Running)));
    }
}

impl PluginGroup for ZoetropePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SetupPlugin)
            .add(BasePlugin)
            .add(AudioPlugin)
            .add(AnimationPlugin)
            .add(CameraPlugin)
            .add(GuiPlugin)
            .add(BluetoothPlugin)
    }
}
