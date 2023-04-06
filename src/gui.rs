use crate::audio::Volume;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

// constants for the different masks
pub const FULL: u8 = 0;
pub const MED: u8 = 1;
pub const LOW: u8 = 2;

#[derive(Resource, Default)]
pub struct ColorSettings {
    pub brightness: f32,
    pub contrast: f32,
    pub saturation: f32,
    pub gamma: f32,
    pub white_balance: f32,
}

#[derive(Component)]
pub struct CameraMaskTag(pub u8);

#[derive(PartialEq, Default)]
enum MaskType {
    #[default]
    None,
    Full,
    Medium,
    Low,
}

#[derive(Resource, Default)]
pub struct CameraMaskSetting(MaskType);

#[derive(Resource, Default)]
pub struct CameraCrosshair(pub bool);

#[derive(Component)]
pub struct CameraCrosshairTag;

#[derive(Resource, Default)]
pub struct UiState {
    pub is_window_open: bool,
}

pub fn gui_full(
    // mut egui_ctx: ResMut<EguiContext>, // old bevy 0.9.1 stuff
    mut ctx: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut mask: ResMut<CameraMaskSetting>,
    mut crosshair: ResMut<CameraCrosshair>,
    mut query: Query<&mut Transform, With<Camera>>,
    mut color_settings: ResMut<ColorSettings>,
    mut volume: ResMut<Volume>,
) {
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
    egui::Window::new("Masks")
        .vscroll(true)
        .open(&mut ui_state.is_window_open)
        .show(ctx.ctx_mut(), |ui| {
            ui.radio_value(&mut mask.0, MaskType::None, "No Mask");
            ui.radio_value(&mut mask.0, MaskType::Full, "Mask 4k");
            ui.radio_value(&mut mask.0, MaskType::Medium, "Mask 1440");
            ui.radio_value(&mut mask.0, MaskType::Low, "Mask 1080");
            ui.checkbox(&mut crosshair.0, "Crosshair");
            if ui.add(egui::Button::new("Re-Center")).clicked() {
                for mut transform in query.iter_mut() {
                    *transform = Transform::from_xyz(0., 0., 100.0).looking_at(Vec3::ZERO, Vec3::Y);
                }
            }
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
}

pub fn gui_change_mask(
    mask: Res<CameraMaskSetting>,
    mut mask_query: Query<(&mut Visibility, &CameraMaskTag)>,
) {
    // MaskType = None -> mask_full && mask_half = INVISIBLE
    // MaskType = Full -> mask_full = VISIBLE && mask_half = INVISIBLE
    // MaskType = Half -> mask_full = INVISIBLE && mask_half = VISIBLE
    for (mut vis, mask_num) in &mut mask_query.iter_mut() {
        match mask.0 {
            // 4k
            MaskType::None => match mask_num.0 {
                FULL => *vis = Visibility::Hidden,
                MED => *vis = Visibility::Hidden,
                LOW => *vis = Visibility::Hidden,
                _ => *vis = Visibility::Hidden,
            },
            MaskType::Full => match mask_num.0 {
                FULL => *vis = Visibility::Visible,
                MED => *vis = Visibility::Hidden,
                LOW => *vis = Visibility::Hidden,
                _ => *vis = Visibility::Hidden,
            },
            // 1440p
            MaskType::Medium => match mask_num.0 {
                FULL => *vis = Visibility::Hidden,
                MED => *vis = Visibility::Visible,
                LOW => *vis = Visibility::Hidden,
                _ => *vis = Visibility::Hidden,
            },
            // 1080p
            MaskType::Low => match mask_num.0 {
                FULL => *vis = Visibility::Hidden,
                MED => *vis = Visibility::Hidden,
                LOW => *vis = Visibility::Visible,
                _ => *vis = Visibility::Hidden,
            },
        }
    }
}

pub fn gui_set_crosshair(
    crosshair: Res<CameraCrosshair>,
    mut cross_query: Query<&mut Visibility, With<CameraCrosshairTag>>,
) {
    for mut vis in &mut cross_query.iter_mut() {
        match crosshair.0 {
            true => *vis = Visibility::Visible,
            false => *vis = Visibility::Hidden,
        }
    }
}

pub fn gui_open(keyboard_input: Res<Input<KeyCode>>, mut ui_state: ResMut<UiState>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        ui_state.is_window_open = !ui_state.is_window_open;
    }
}

pub fn gui_camera_control(
    keyboard_input: Res<Input<KeyCode>>,
    // mut movement: ResMut<CameraMovement>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let mut transform = query.single_mut();
    let mut movement_speed: f32 = 1.;
    if keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::RShift) {
        movement_speed = 3.;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        transform.translation.x -= movement_speed;
    } else if keyboard_input.pressed(KeyCode::Right) {
        transform.translation.x += movement_speed;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        transform.translation.y += movement_speed;
    } else if keyboard_input.pressed(KeyCode::Down) {
        transform.translation.y -= movement_speed;
    }
    if keyboard_input.pressed(KeyCode::PageUp) {
        transform.scale -= movement_speed / 500.0;
    } else if keyboard_input.pressed(KeyCode::PageDown) {
        transform.scale += movement_speed / 500.0;
    }
}
