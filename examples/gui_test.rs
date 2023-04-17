use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use uhdrtz::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<UiState>()
        .init_resource::<CameraCrosshair>()
        .init_resource::<ColorSettings>()
        .insert_resource(Volume(0.5))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: bevy::window::WindowMode::BorderlessFullscreen,
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_startup_system(set_background_color)
        .add_startup_system(configure_ui_state)
        .add_system(gui_full)
        .add_system(gui_open)
        .add_system(gui_set_crosshair)
        .add_system(gui_camera_control)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
pub struct ZoetropeImage;

fn configure_ui_state(mut ui_state: ResMut<UiState>) {
    ui_state.is_window_open = true;
}

fn set_background_color(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands
        .spawn(bevy::sprite::MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(800.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_xyz(0., 0., 50.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(ZoetropeImage);
    commands
        .spawn(SpriteBundle {
            texture: server.load("xhair.png"),
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            visibility: Visibility::Hidden,
            ..default()
        })
        .insert(CameraCrosshairTag);
}
