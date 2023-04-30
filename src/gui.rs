use crate::{
    audio::Volume,
    camera::{CameraSetting, ColorSettings, VideoStream},
    zoetrope::{Slices, ZoetropeAnimationThresholdSpeed},
};
use bevy::{prelude::*, render::color};
use bevy_egui::{egui, EguiContexts};

use egui::color_picker::color_picker_color32;
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
            fn send_camera_setting(cam: &VideoStream, id: KnownCameraControl, value: i64) {
                if let Err(why) = cam.op_tx.send(CameraSetting {
                    id: id,
                    control: ControlValueSetter::Integer(value),
                }) {
                    eprintln!("{}", why);
                }
            }

            // Brightness
            if ui
                .add(
                    egui::Slider::new(&mut color_settings.brightness, -15..=15)
                        .text("Brightness")
                        .show_value(true),
                )
                .changed()
            {
                send_camera_setting(
                    cam,
                    KnownCameraControl::Brightness,
                    color_settings.brightness.into(),
                );
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
                send_camera_setting(
                    cam,
                    KnownCameraControl::Contrast,
                    color_settings.contrast.into(),
                );
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
                send_camera_setting(
                    cam,
                    KnownCameraControl::Saturation,
                    color_settings.saturation.into(),
                );
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
                send_camera_setting(cam, KnownCameraControl::Gamma, color_settings.gamma.into());
            }

            // Gain
            // if ui
            //     .add(
            //         egui::Slider::new(&mut color_settings.gain, 0..=100)
            //             .text("Gain")
            //             .show_value(true),
            //     )
            //     .changed()
            // {
            //     send_camera_setting(cam, KnownCameraControl::Gain, color_settings.gain.into());
            // }

            // White Balance
            // if ui
            //     .add(
            //         egui::Slider::new(&mut color_settings.white_balance, 1000..=10000)
            //             .text("White Balance")
            //             .show_value(true),
            //     )
            //     .changed()
            // {
            //     send_camera_setting(
            //         cam,
            //         KnownCameraControl::WhiteBalance,
            //         color_settings.white_balance.into(),
            //     );
            // }

            // Sharpness
            if ui
                .add(
                    egui::Slider::new(&mut color_settings.sharpness, 0..=127)
                        .text("Sharpness")
                        .show_value(true),
                )
                .changed()
            {
                send_camera_setting(
                    cam,
                    KnownCameraControl::Sharpness,
                    color_settings.sharpness.into(),
                );
            }

            if ui.add(egui::Button::new("Reset to Defaults")).clicked() {
                const BRIGHTNESS_DEFAULT: i8 = 0;
                const CONTRAST_DEFAULT: u8 = 15;
                const SATURATION_DEFAULT: u8 = 32;
                const GAMMA_DEFAULT: u16 = 220;
                const SHARPNESS_DEFAULT: u8 = 16;

                color_settings.brightness = BRIGHTNESS_DEFAULT;
                send_camera_setting(
                    cam,
                    KnownCameraControl::Brightness,
                    BRIGHTNESS_DEFAULT.into(),
                );

                color_settings.contrast = CONTRAST_DEFAULT;
                send_camera_setting(cam, KnownCameraControl::Contrast, CONTRAST_DEFAULT.into());

                color_settings.saturation = SATURATION_DEFAULT;
                send_camera_setting(
                    cam,
                    KnownCameraControl::Saturation,
                    SATURATION_DEFAULT.into(),
                );

                color_settings.gamma = GAMMA_DEFAULT;
                send_camera_setting(cam, KnownCameraControl::Gamma, GAMMA_DEFAULT.into());

                // send_camera_setting(cam, KnownCameraControl::Gain, 0);

                // send_camera_setting(cam, KnownCameraControl::WhiteBalance, 5000);

                color_settings.sharpness = SHARPNESS_DEFAULT;
                send_camera_setting(cam, KnownCameraControl::Sharpness, SHARPNESS_DEFAULT.into());
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
