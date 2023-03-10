use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use uhdrtz::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .init_resource::<UiState>()
        .init_resource::<CameraCrosshair>()
        .init_resource::<CameraMaskSetting>()
        .init_resource::<ColorSettings>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            },
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_startup_system(set_background_color)
        .add_startup_system(configure_ui_state)
        .add_system(gui_full)
        .add_system(gui_open)
        .add_system(gui_set_crosshair)
        .add_system(gui_change_mask)
        .add_system(gui_camera_control)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn configure_ui_state(mut ui_state: ResMut<UiState>) {
    ui_state.is_window_open = true;
}

fn set_background_color(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands
        .spawn(SpriteBundle {
            texture: server.load("mask_full.png"),
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            visibility: Visibility::INVISIBLE,
            ..default()
        })
        .insert(CameraMaskTag(0));
    commands
        .spawn(SpriteBundle {
            texture: server.load("mask_half.png"),
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            visibility: Visibility::INVISIBLE,
            ..default()
        })
        .insert(CameraMaskTag(1));
    commands
        .spawn(SpriteBundle {
            texture: server.load("xhair.png"),
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            visibility: Visibility::INVISIBLE,
            ..default()
        })
        .insert(CameraCrosshairTag);
}
