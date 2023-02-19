// future location for the implementation of the bevy_egui systems for allowing
// access to users to modify the parameters of the overall physical system.

// This will be camera controls as well as image controls to change the look
// and maybe feel of the project as a whole.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

#[derive(Resource, Default)]
pub struct UiState {
    pub is_window_open: bool,
}

pub fn ui_test(mut egui_ctx: ResMut<EguiContext>, mut ui_state: ResMut<UiState>) {
    // Remove this section when fully implementing
    let mut my_f32 = 0.0;
    let mut crosshair = true;
    #[derive(PartialEq)]
    enum Enum {
        First,
        Second,
        Third,
    }
    let mut my_enum = Enum::First;
    // End of remove section
    //Unsure if UiState needs to be initialized somewhere)
    egui::Window::new("Settings")
        .vscroll(true)
        .open(&mut ui_state.is_window_open) //unsure if I can remove this part or not (might depend on button press)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.label("Color Section");
            ui.add(
                egui::Slider::new(&mut my_f32, 0.0..=100.0)
                    .text("Hue")
                    .show_value(true),
            );
            ui.checkbox(&mut crosshair, "Crosshair");
            ui.separator();
            ui.label("Select Desired Mask");
            ui.radio_value(&mut my_enum, Enum::First, "No Mask");
            ui.radio_value(&mut my_enum, Enum::Second, "Mask 1");
            ui.radio_value(&mut my_enum, Enum::Third, "Mask 2");
        });

    //Should implement a slider. Got not clue for what tho
    // if ui.add(egui::DragValue::new(camera.get_mut_i64_control(known_control).unwrap(),)).changed() { //I belive this checks to see if a part of known_controls has changed
    //     let _ = camera.operating_tx.try_send(CameraOperation::Control { //Attempts to send the new change to the camera
    //         id: known_control.clone(),
    //         control: camera.controls.get(known_control).unwrap().clone(),
    //     });
    // };
}

pub fn open_window(keyboard_input: Res<Input<KeyCode>>, mut ui_state: ResMut<UiState>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        ui_state.is_window_open = !ui_state.is_window_open;
    }
}
