// future location for the implementation of the bevy_egui systems for allowing
// access to users to modify the parameters of the overall physical system.

// This will be camera controls as well as image controls to change the look
// and maybe feel of the project as a whole.

//use crate::zoetrope::ZoetropeImage;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

#[derive(Component)]
pub struct MoveX(pub f64);
#[derive(Component)]
pub struct MoveY(pub f64);
#[derive(Component)]
pub struct MoveZoom(pub f64);

pub const FULL: u8 = 0;
pub const HALF: u8 = 1;

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
    mut x_pos: Mut<MoveX>,
) {
    // Remove this section when fully implementing
    let mut my_f32 = 0.0;
    // End of remove section
    //Unsure if UiState needs to be initialized somewhere)
    egui::Window::new("Effects")
        .vscroll(true)
        .open(&mut ui_state.is_window_open) //unsure if I can remove this part or not (might depend on button press)
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
            ui.add(
                egui::Slider::new(&mut x_pos, 0.0..=100.0)
                    .text("X Movement")
                    .show_value(true),
            )
        });

    //Should implement a slider. Got not clue for what tho
    // if ui.add(egui::DragValue::new(camera.get_mut_i64_control(known_control).unwrap(),)).changed() { //I belive this checks to see if a part of known_controls has changed
    //     let _ = camera.operating_tx.try_send(CameraOperation::Control { //Attempts to send the new change to the camera
    //         id: known_control.clone(),
    //         control: camera.controls.get(known_control).unwrap().clone(),
    //     });
    // };
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

pub fn move_camera(x_pos: MoveX, y_pos: MoveY, zoom_val: MoveZoom) {}

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
