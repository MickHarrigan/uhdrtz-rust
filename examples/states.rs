use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use egui::{FontId, RichText};
use uhdrtz::prelude::*;

/// TODO:
///    - Have a system run @ OnEnter(RunningStates::Setup) that gets the cameras and their properties as well as the bluetooth devices around (Arduino/RotaryArduino).
///    - run @ OnExit(RunningStates::Setup) a way to write the selected config to a file?

#[derive(Debug, PartialEq)]
enum Enum {
    First,
    Second,
    Third,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "UHDRTZ Setup".to_string(),
                position: WindowPosition::Centered {
                    0: MonitorSelection::Primary,
                },
                ..default()
            }),
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_state::<RunningStates>()
        .add_system(setup_menu.in_set(OnUpdate(RunningStates::Setup)))
        .add_system(cleanup_menu.in_schedule(OnExit(RunningStates::Setup)))
        .run();
}

fn setup_menu(mut ctx: EguiContexts) {
    let mut selected = Enum::First;
    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label(RichText::new("Large").font(FontId::proportional(40.0)));
        });

        egui::Grid::new("my_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                // this is where the different items are defined
                // start with a `ui.add()`
                ui.add(egui::Label::new("Camera"));
                egui::ComboBox::from_label("Select one!")
                    .selected_text(format!("{:?}", selected))
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.set_min_width(50.0);
                        ui.selectable_value(&mut selected, Enum::First, "First");
                        ui.selectable_value(&mut selected, Enum::Second, "Second");
                        ui.selectable_value(&mut selected, Enum::Third, "Third");
                    });
                // end with a `ui.end_row()`
                ui.end_row();
            });
    });
}

fn cleanup_menu() {
    info!("Cleaned up!");
}
