use crate::components::{VideoFrame, VideoStream};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
use nokhwa::pixel_format::RgbAFormat;
use nokhwa::utils::{CameraFormat, FrameFormat, RequestedFormat, RequestedFormatType, Resolution};

#[derive(Resource, Default)]
pub struct UiState {
    pub is_window_open: bool,
}

pub fn handle_video_frame(
    cam_query: Query<&mut VideoStream>,
    image: Res<VideoFrame>,
    mut images: ResMut<Assets<Image>>,
    mut tex_query: Query<&mut Handle<Image>>,
) {
    for camera in cam_query.iter() {
        while let Some(img) = camera.image_rx.drain().last() {
            for mut tex in &mut tex_query.iter_mut() {
                *tex = images.set(
                    &image.0,
                    Image::new_fill(
                        Extent3d {
                            width: 3840,
                            height: 2160,
                            depth_or_array_layers: 1,
                        },
                        TextureDimension::D2,
                        &img,
                        TextureFormat::Rgba8UnormSrgb,
                    ),
                );
            }
        }
    }
}

pub fn camera_rotation(time: Res<Time>, mut query: Query<&mut Transform, With<Camera>>) {
    for mut transform in query.iter_mut() {
        // https://github.com/bevyengine/bevy/blob/main/examples/2d/rotation.rs
        transform.rotate_z(time.delta_seconds());
    }
}

pub fn setup_physical_camera(mut commands: Commands, video_images: Res<VideoFrame>) {
    // next up is to open a camera (both physical camera for taking an image as well as the logical bevy one that looks at a plane)
    // then open a stream from the camera with the right settings
    // then constantly (read: every frame of the "game") get and image from the camera
    // and to display that image to a plane that a 2d camera is looking at

    let cam = VideoStream::new(
        0,
        RequestedFormat::new::<RgbAFormat>(RequestedFormatType::Closest(CameraFormat::new(
            Resolution::new(3840, 2160),
            FrameFormat::MJPEG,
            30,
        ))),
    )
    .unwrap();

    commands
        .spawn(Camera2dBundle {
            transform: Transform::from_xyz(0., 0., 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(cam);

    commands.spawn(SpriteBundle {
        texture: video_images.0.clone_weak(),
        transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y), // TODO: update the transform
        ..default()
    });
}

pub fn ui_test(mut egui_ctx: ResMut<EguiContext>, mut ui_state: ResMut<UiState>) {
    let mut my_f32 = 0.0;
    //Unsure if UiState needs to be initialized somewhere)
    egui::Window::new("Window")
        .vscroll(true)
        .open(&mut ui_state.is_window_open) //unsure if I can remove this part or not (might depend on button press)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.label("What the fuck is up kyle!");
            ui.add(egui::Slider::new(&mut my_f32, 0.0..=100.0).text("Test Value").show_value(true));
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
