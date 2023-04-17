use crate::audio::Volume;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use nokhwa::utils::Resolution;

#[derive(Resource, Default)]
pub struct ColorSettings {
    pub brightness: f32,
    pub contrast: f32,
    pub saturation: f32,
    pub gamma: f32,
    pub white_balance: f32,
}

#[derive(Resource, Default)]
pub struct CameraCrosshair(pub bool);

#[derive(Component)]
pub struct CameraCrosshairTag;

#[derive(Resource, Default)]
pub struct UiState {
    pub is_window_open: bool,
}

pub fn gui_full(
    mut ctx: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut color_settings: ResMut<ColorSettings>,
    mut volume: ResMut<Volume>,
    mut query: Query<&mut Transform, With<Camera>>,
    window_query: Query<&Window>,
) {
    let window = window_query.single();
    let mut transform = query.single_mut();
    egui::Window::new("Effects")
        .vscroll(true)
        .open(&mut ui_state.is_window_open)
        .show(ctx.ctx_mut(), |ui| {
            ui.add(
                egui::Slider::new(&mut color_settings.brightness, 0.0..=100.0)
                    .text("Brightness")
                    .show_value(true),
            );
            ui.add(
                egui::Slider::new(&mut color_settings.contrast, 0.0..=100.0)
                    .text("Contrast")
                    .show_value(true),
            );
            ui.add(
                egui::Slider::new(&mut color_settings.saturation, 0.0..=100.0)
                    .text("Saturation")
                    .show_value(true),
            );
            ui.add(
                egui::Slider::new(&mut color_settings.gamma, 0.0..=100.0)
                    .text("Gamma")
                    .show_value(true),
            );
            ui.add(
                egui::Slider::new(&mut color_settings.white_balance, 0.0..=100.0)
                    .text("White Balance")
                    .show_value(true),
            );
        });

    egui::Window::new("Volume")
        .open(&mut ui_state.is_window_open)
        .show(ctx.ctx_mut(), |ui| {
            ui.add(
                egui::Slider::new(&mut volume.0, 0.0..=1.0)
                    .text("Volume")
                    .show_value(true),
            );
        });

    egui::Window::new("Presets")
        .open(&mut ui_state.is_window_open)
        .show(ctx.ctx_mut(), |ui| {
            if ui.add(egui::Button::new("Re-Center")).clicked() {
                *transform = Transform::from_xyz(0., 0., 100.0).looking_at(Vec3::ZERO, Vec3::Y);
            }
            if ui.add(egui::Button::new("Semi-Circle")).clicked() {
                transform.translation.y = window.resolution.height() / 2.0;
            }
        });
}

pub fn gui_set_crosshair(
    mut cross_query: Query<&mut Visibility, With<CameraCrosshairTag>>,
    ui_state: Res<UiState>,
) {
    if ui_state.is_window_open {
        *cross_query.single_mut() = Visibility::Visible;
    } else {
        *cross_query.single_mut() = Visibility::Hidden;
    }
}

pub fn gui_open(keyboard_input: Res<Input<KeyCode>>, mut ui_state: ResMut<UiState>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        ui_state.is_window_open = !ui_state.is_window_open;
    }
}

pub fn gui_camera_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let mut transform = query.single_mut();
    let mut movement_speed: f32 = 1.;
    if keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::RShift) {
        movement_speed = 3.;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        transform.translation.x += movement_speed;
    } else if keyboard_input.pressed(KeyCode::Right) {
        transform.translation.x -= movement_speed;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        transform.translation.y -= movement_speed;
    } else if keyboard_input.pressed(KeyCode::Down) {
        transform.translation.y += movement_speed;
    }
    if keyboard_input.pressed(KeyCode::PageUp) {
        transform.scale -= movement_speed / 500.0;
    } else if keyboard_input.pressed(KeyCode::PageDown) {
        transform.scale += movement_speed / 500.0;
    }
}
