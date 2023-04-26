use crate::{
    audio::Volume,
    camera::{CameraSetting, ColorSettings, VideoStream},
    zoetrope::{Slices, ZoetropeAnimationThresholdSpeed},
};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use nokhwa::utils::{ControlValueSetter, KnownCameraControl};

#[derive(Resource, Default)]
pub struct CameraCrosshair(pub bool);

#[derive(Component)]
pub struct CameraCrosshairTag;

#[derive(Resource, Default)]
pub struct UiState {
    pub is_window_open: bool,
}

pub struct CameraControlEvent;

pub fn gui_full(
    mut ctx: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut color_settings: ResMut<ColorSettings>,
    mut volume: ResMut<Volume>,
    slices: Res<Slices>,
    mut query: Query<&mut Transform, With<Camera>>,
    window_query: Query<&Window>,
    mut threshold: ResMut<ZoetropeAnimationThresholdSpeed>,
    mut event_writer: EventWriter<CameraControlEvent>,
    cam_query: Query<&VideoStream>,
) {
    let window = window_query.single();
    let mut transform = query.single_mut();
    let cam = cam_query.single();
    egui::Window::new("Effects")
        .vscroll(true)
        .open(&mut ui_state.is_window_open)
        .show(ctx.ctx_mut(), |ui| {
            // Brightness
            if ui
                .add(
                    egui::Slider::new(&mut color_settings.brightness, -15..=15)
                        .text("Brightness")
                        .show_value(true),
                )
                .changed()
            {
                if let Err(why) = cam.op_tx.send(CameraSetting {
                    id: KnownCameraControl::Brightness,
                    control: ControlValueSetter::Integer(color_settings.brightness.into()),
                }) {
                    eprintln!("{}", why);
                }
            }

            // Contrast
            if ui
                .add(
                    egui::Slider::new(&mut color_settings.contrast, 0..=30)
                        .text("Contrast")
                        .show_value(true),
                )
                .changed()
            {
                if let Err(why) = cam.op_tx.send(CameraSetting {
                    id: KnownCameraControl::Contrast,
                    control: ControlValueSetter::Integer(color_settings.contrast.into()),
                }) {
                    eprintln!("{}", why);
                }
            }

            // Saturation
            if ui
                .add(
                    egui::Slider::new(&mut color_settings.saturation, 0..=60)
                        .text("Saturation")
                        .show_value(true),
                )
                .changed()
            {
                if let Err(why) = cam.op_tx.send(CameraSetting {
                    id: KnownCameraControl::Saturation,
                    control: ControlValueSetter::Integer(color_settings.saturation.into()),
                }) {
                    eprintln!("{}", why);
                }
            }

            // Gamma
            if ui
                .add(
                    egui::Slider::new(&mut color_settings.gamma, 40..=500)
                        .text("Gamma")
                        .show_value(true),
                )
                .changed()
            {
                if let Err(why) = cam.op_tx.send(CameraSetting {
                    id: KnownCameraControl::Gamma,
                    control: ControlValueSetter::Integer(color_settings.gamma.into()),
                }) {
                    eprintln!("{}", why);
                }
            }

            // Gain
            if ui
                .add(
                    egui::Slider::new(&mut color_settings.gain, 0..=100)
                        .text("Gain")
                        .show_value(true),
                )
                .changed()
            {
                if let Err(why) = cam.op_tx.send(CameraSetting {
                    id: KnownCameraControl::Gain,
                    control: ControlValueSetter::Integer(color_settings.gain.into()),
                }) {
                    eprintln!("{}", why);
                }
            }

            // White Balance
            if ui
                .add(
                    egui::Slider::new(&mut color_settings.white_balance, 1000..=10000)
                        .text("White Balance")
                        .show_value(true),
                )
                .changed()
            {
                if let Err(why) = cam.op_tx.send(CameraSetting {
                    id: KnownCameraControl::WhiteBalance,
                    control: ControlValueSetter::Integer(color_settings.white_balance.into()),
                }) {
                    eprintln!("{}", why);
                }
            }

            // Sharpness
            if ui
                .add(
                    egui::Slider::new(&mut color_settings.sharpness, 0..=127)
                        .text("Sharpness")
                        .show_value(true),
                )
                .changed()
            {
                if let Err(why) = cam.op_tx.send(CameraSetting {
                    id: KnownCameraControl::Sharpness,
                    control: ControlValueSetter::Integer(color_settings.sharpness.into()),
                }) {
                    eprintln!("{}", why);
                }
            }

            // Zoom
            // if ui
            //     .add(
            //         egui::Slider::new(&mut color_settings.zoom, 100..=800)
            //             .text("Zoom")
            //             .show_value(true),
            //     )
            //     .changed()
            // {
            //     event_writer.send(CameraControlEvent);
            // }

            // Auto Exposure
            // if ui
            //     .add(egui::Checkbox::new(
            //         &mut color_settings.auto_exposure,
            //         "Auto Exposure",
            //     ))
            //     .changed()
            // {
            //     event_writer.send(CameraControlEvent);
            // }

            // TODO: add button to reset values to default
            // if ui.add(egui::Button::new("Reset")).clicked() {
            //     color_settings = ColorSettings::default();
            // }
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
    egui::Window::new("Debug value inspector")
        .open(&mut ui_state.is_window_open)
        .show(ctx.ctx_mut(), |ui| {
            ui.add(egui::Label::new(format!("{}", slices.0)));
        });

    egui::Window::new("Presets")
        .open(&mut ui_state.is_window_open)
        .show(ctx.ctx_mut(), |ui| {
            if ui.add(egui::Button::new("Re-Center")).clicked() {
                *transform = Transform::from_xyz(0., 0., 100.0).looking_at(Vec3::ZERO, Vec3::Y);
            }
            if ui.add(egui::Button::new("Semi-Circle")).clicked() {
                *transform = Transform::from_xyz(0., 0., 100.0).looking_at(Vec3::ZERO, Vec3::Y);
                transform.translation.y = window.resolution.height() / 2.0;
                transform.scale = Vec3::new(0.825, 0.825, 1.);
            }
        });

    egui::Window::new("Rotational Speed Threshold")
        .open(&mut ui_state.is_window_open)
        .show(ctx.ctx_mut(), |ui| {
            ui.add(
                egui::Slider::new(&mut threshold.0, 1..=20)
                    .text("Required Rotational Speed to animate fully")
                    .show_value(true),
            );
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
