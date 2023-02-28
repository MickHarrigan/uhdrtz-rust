// future location for the implementation of the bevy_egui systems for allowing
// access to users to modify the parameters of the overall physical system.

// This will be camera controls as well as image controls to change the look
// and maybe feel of the project as a whole.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub const FULL: u8 = 0;
pub const HALF: u8 = 1;

#[derive(Resource, Default)]
pub struct Movement(pub f32, pub f32, pub f32);

#[derive(Component)]
pub struct MaskImage(pub u8);

#[derive(PartialEq, Default)]
enum MaskType {
    #[default]
    None,
    Full,
    Half,
}

#[derive(Resource, Default)]
pub struct MaskSetting(MaskType);

#[derive(Resource, Default)]
pub struct Crosshair(bool);

#[derive(Component)]
pub struct CrossImage;

#[derive(Resource, Default)]
pub struct UiState {
    pub is_window_open: bool,
}

pub fn ui_test(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut mask: ResMut<MaskSetting>,
    mut crosshair: ResMut<Crosshair>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    // Remove this section when fully implementing
    let mut my_f32 = 0.0;
    // End of remove section
    egui::Window::new("Effects")
        .vscroll(true)
        .open(&mut ui_state.is_window_open)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.label("Color Section");
            ui.add(
                egui::Slider::new(&mut my_f32, 0.0..=100.0)
                    .text("Hue")
                    .show_value(true),
            );
        });
    egui::Window::new("Masks")
        .vscroll(true)
        .open(&mut ui_state.is_window_open)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.radio_value(&mut mask.0, MaskType::None, "No Mask");
            ui.radio_value(&mut mask.0, MaskType::Full, "Mask Full");
            ui.radio_value(&mut mask.0, MaskType::Half, "Mask Half");
            ui.checkbox(&mut crosshair.0, "Crosshair");
            if ui.add(egui::Button::new("Re-Center")).clicked() {
                for mut transform in query.iter_mut() {
                    transform.translation.x = 0.0;
                    transform.translation.y = 0.0;
                    transform.scale.x = 1.0;
                    transform.scale.y = 1.0;
                }
            }
        });
}

pub fn change_mask(mask: Res<MaskSetting>, mut mask_query: Query<(&mut Visibility, &MaskImage)>) {
    // MaskType = None -> mask_full && mask_half = INVISIBLE
    // MaskType = Full -> mask_full = VISIBLE && mask_half = INVISIBLE
    // MaskType = Half -> mask_full = INVISIBLE && mask_half = VISIBLE
    for (mut vis, mask_num) in &mut mask_query.iter_mut() {
        match mask.0 {
            MaskType::None => match mask_num.0 {
                FULL => *vis = Visibility::INVISIBLE,
                HALF => *vis = Visibility::INVISIBLE,
                _ => *vis = Visibility::INVISIBLE,
            },
            MaskType::Full => match mask_num.0 {
                FULL => *vis = Visibility::VISIBLE,
                HALF => *vis = Visibility::INVISIBLE,
                _ => *vis = Visibility::INVISIBLE,
            },
            MaskType::Half => match mask_num.0 {
                FULL => *vis = Visibility::INVISIBLE,
                HALF => *vis = Visibility::VISIBLE,
                _ => *vis = Visibility::INVISIBLE,
            },
        }
    }
}

pub fn logical_camera_movement(
    mut query: Query<&mut Transform, With<Camera>>,
    movement: Res<Movement>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.x += movement.0;
        transform.translation.y += movement.1;
        transform.scale += movement.2;
    }
}

pub fn set_crosshair(
    crosshair: Res<Crosshair>,
    mut cross_query: Query<&mut Visibility, With<CrossImage>>,
) {
    for mut vis in &mut cross_query.iter_mut() {
        match crosshair.0 {
            true => *vis = Visibility::VISIBLE,
            false => *vis = Visibility::INVISIBLE,
        }
    }
}

pub fn open_window(keyboard_input: Res<Input<KeyCode>>, mut ui_state: ResMut<UiState>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        ui_state.is_window_open = !ui_state.is_window_open;
    }
}

pub fn camera_control(keyboard_input: Res<Input<KeyCode>>, mut movement: ResMut<Movement>) {
    let movement_speed: f32 = 0.25;
    if keyboard_input.pressed(KeyCode::Left) {
        movement.0 -= movement_speed;
    } else if keyboard_input.pressed(KeyCode::Right) {
        movement.0 += movement_speed;
    } else {
        movement.0 = 0.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        movement.1 += movement_speed;
    } else if keyboard_input.pressed(KeyCode::Down) {
        movement.1 -= movement_speed;
    } else {
        movement.1 = 0.0;
    }

    if keyboard_input.pressed(KeyCode::PageUp) {
        movement.2 -= movement_speed / 1000.0;
    } else if keyboard_input.pressed(KeyCode::PageDown) {
        movement.2 += movement_speed / 1000.0;
    } else {
        movement.2 = 0.0;
    }
}
