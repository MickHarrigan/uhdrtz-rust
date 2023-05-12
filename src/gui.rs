use crate::{
    audio::VolumeEvent,
    camera::{
        reset_camera_controls, send_camera_setting, CameraSetting, ColorSettings, VideoStream,
    },
    zoetrope::{
        Direction, RotationDirection, Slices, ZoetropeAnimationThresholdSpeed, ZoetropeImage,
        TOP_BAR_SIZE,
    },
};
use bevy::{prelude::*, sprite::Mesh2dHandle};
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

#[derive(Resource, Default)]
pub struct Volume(f64);

pub fn gui_full(
    mut ctx: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut color_settings: ResMut<ColorSettings>,
    mut vol_event: EventWriter<VolumeEvent>,
    mut vol: ResMut<Volume>,
    slices: Res<Slices>,
    mut query: Query<&mut Transform, With<Camera>>,
    window_query: Query<&Window>,
    mut threshold: ResMut<ZoetropeAnimationThresholdSpeed>,
    cam_query: Query<&VideoStream>,
    mut circle: Query<&mut Mesh2dHandle, With<ZoetropeImage>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut directions: ResMut<RotationDirection>,
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

            // Zoom
            if ui
                .add(
                    egui::Slider::new(&mut color_settings.zoom, 100..=800)
                        .text("Zoom")
                        .show_value(true),
                )
                .changed()
            {
                send_camera_setting(
                    cam,
                    KnownCameraControl::Other(10094861),
                    color_settings.zoom.into(),
                );
            }

            // Tilt
            if ui
                .add(
                    egui::Slider::new(&mut color_settings.tilt, -648000..=648000)
                        .step_by(3600.0)
                        .text("Tilt")
                        .show_value(true),
                )
                .changed()
            {
                send_camera_setting(
                    cam,
                    KnownCameraControl::Other(10094857),
                    color_settings.tilt.into(),
                );
            }

            // Pan
            if ui
                .add(
                    egui::Slider::new(&mut color_settings.pan, -648000..=648000)
                        .step_by(3600.0)
                        .text("Pan")
                        .show_value(true),
                )
                .changed()
            {
                send_camera_setting(
                    cam,
                    KnownCameraControl::Other(10094856),
                    color_settings.pan.into(),
                );
            }

            if ui.add(egui::Button::new("Reset to Defaults")).clicked() {
                reset_camera_controls(color_settings, cam);
            }
        });

    egui::Window::new("Volume")
        .open(&mut ui_state.is_window_open)
        .show(ctx.ctx_mut(), |ui| {
            let mut thing: f64 = vol.0;
            if ui
                .add(
                    egui::Slider::new(&mut thing, 0.0..=1.0)
                        .text("Volume")
                        .show_value(true),
                )
                .changed()
            {
                vol.0 = thing;
                vol_event.send(VolumeEvent(thing));
            }
        });

    egui::Window::new("Rotation and Audio Direction")
        .open(&mut ui_state.is_window_open)
        .show(ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.radio_value(&mut directions.audio, Direction::CW, "Clockwise");
                ui.radio_value(&mut directions.audio, Direction::CCW, "Counter Clockwise");
                ui.add(egui::Label::new("Audio Direction"));
            });
            ui.horizontal(|ui| {
                ui.radio_value(&mut directions.animation, Direction::CW, "Clockwise");
                ui.radio_value(
                    &mut directions.animation,
                    Direction::CCW,
                    "Counter Clockwise",
                );
                ui.add(egui::Label::new("Crank/Animation Direction"));
            });
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
                let size = (window.height() / 2.).ceil() + TOP_BAR_SIZE as f32;
                *circle.single_mut() = meshes.add(shape::Circle::new(size).into()).into();
                *transform = Transform::from_xyz(0., 0., 100.0).looking_at(Vec3::ZERO, Vec3::Y);
            }
            if ui.add(egui::Button::new("Semi-Circle")).clicked() {
                let size = ((window.width() / 2.0) * 0.99).ceil();
                *circle.single_mut() = meshes.add(shape::Circle::new(size).into()).into();
                *transform = Transform::from_xyz(0., 0., 100.0).looking_at(Vec3::ZERO, Vec3::Y);
                transform.translation.y = window.resolution.height() / 2.0;
            }
            if ui.add(egui::Button::new("Right")).clicked() {
                let size = (window.height()).ceil();
                *circle.single_mut() = meshes.add(shape::Circle::new(size).into()).into();
                let location = ((window.width()) / 2.0).ceil();
                *transform = Transform::from_xyz(0., 0., 100.0).looking_at(Vec3::ZERO, Vec3::Y);
                transform.translation.y = window.resolution.height() / 2.0;
                transform.translation.x = -location;
            }
            if ui.add(egui::Button::new("Left")).clicked() {
                let size = (window.height()).ceil();
                *circle.single_mut() = meshes.add(shape::Circle::new(size).into()).into();
                let location = ((window.width()) / 2.0).ceil();
                *transform = Transform::from_xyz(0., 0., 100.0).looking_at(Vec3::ZERO, Vec3::Y);
                transform.translation.y = window.resolution.height() / 2.0;
                transform.translation.x = location;
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

pub fn cursor_visibility(mut windows: Query<&mut Window>, ui_state: Res<UiState>) {
    let mut window = windows.get_single_mut().unwrap();
    if ui_state.is_window_open {
        window.cursor.visible = true;
    } else {
        window.cursor.visible = false;
    }
}
