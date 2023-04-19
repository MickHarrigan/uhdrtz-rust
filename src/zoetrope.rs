use std::f32::consts::PI;

use crate::bluetooth::RotationInterval;
use crate::camera::VideoStream;
use crate::prelude::CameraCrosshairTag;
use crate::setup::Settings;
use bevy::prelude::*;
use nokhwa::pixel_format::RgbAFormat;
use nokhwa::utils::{CameraFormat, FrameFormat, RequestedFormat, RequestedFormatType};

pub const TOP_BAR_SIZE: u32 = 12;

#[derive(Component)]
pub struct ZoetropeImage;

#[derive(Resource)]
pub struct ZoetropeAnimationThresholdSpeed(pub i8);

#[derive(Resource)]
pub struct Counter(pub u8);

#[derive(Resource)]
pub struct Slices(pub u8);

pub fn zoetrope_setup(
    mut commands: Commands,
    // video_images: Res<VideoFrame>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<Settings>,
    server: Res<AssetServer>,
    windows: Query<&Window>,
) {
    // next up is to open a camera (both physical camera for taking an image as well as the logical bevy one that looks at a plane)
    // then open a stream from the camera with the right settings
    // then constantly (read: every frame of the "game") get and image from the camera
    // and to display that image to a plane that a 2d camera is looking at

    let cam = VideoStream::new(
        settings.camera.clone(),
        RequestedFormat::new::<RgbAFormat>(RequestedFormatType::Closest(CameraFormat::new(
            settings.resolution,
            FrameFormat::MJPEG,
            settings.frame_rate,
        ))),
    )
    .unwrap();

    let size = (windows.single().height() / 2.).ceil() + TOP_BAR_SIZE as f32;

    commands
        .spawn(Camera2dBundle {
            transform: Transform::from_xyz(0., 0., 100.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(cam);

    commands
        .spawn(bevy::sprite::MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(size).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_xyz(0., 0., -1.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(ZoetropeImage);

    commands
        .spawn(SpriteBundle {
            texture: server.load("xhair.png"),
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            visibility: Visibility::Hidden,
            ..default()
        })
        .insert(CameraCrosshairTag);
}

pub fn zoetrope_animation(
    mut query: Query<&mut Transform, With<ZoetropeImage>>,
    rotation: Res<RotationInterval>,
    max: Res<ZoetropeAnimationThresholdSpeed>,
    slices: Res<Slices>,
) {
    for mut transform in query.iter_mut() {
        let val: f32;
        // rotation is an i8
        // need to get it to an f32
        if rotation.0 >= max.0 {
            val = 1.0;
        } else if rotation.0 <= -max.0 {
            val = -1.0;
        } else {
            val = (rotation.0 as f32 / max.0 as f32).into();
        }
        // PI / 12.0 should be tied to the framerate (slices) of the art in question
        transform.rotate_z((2. * PI / slices.0 as f32) * val);
    }
}

#[allow(dead_code)]
pub fn zoetrope_animation_keyboard(
    // mut query: Query<&mut Transform, With<ZoetropeImage>>,
    mut query: Query<&mut Transform, With<ZoetropeImage>>,
    input: Res<Input<KeyCode>>,
) {
    for mut transform in query.iter_mut() {
        let mut rate: f32 = 0.5;
        if input.pressed(KeyCode::LShift) {
            rate = 1.0;
        }
        if input.pressed(KeyCode::RShift) {
            rate = 0.75;
        }
        if input.pressed(KeyCode::Q) {
            transform.rotate_z(-PI / 12.0 * rate);
        } else if input.pressed(KeyCode::E) {
            transform.rotate_z(PI / 12.0 * rate);
        } else {
            transform.rotate_z(0.);
        }
    }
}

pub fn zoetrope_next_camera_frame(
    cam_query: Query<&mut VideoStream>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mat_query: Query<&Handle<ColorMaterial>, With<ZoetropeImage>>,
) {
    let camera = cam_query.single();
    if let Some(image) = camera.image_rx.drain().last() {
        let mat = mat_query.single();
        if let Some(material) = materials.get_mut(&mat) {
            material.texture = Some(images.add(image));
        }
    }
}

pub fn zoetrope_next_frame_static(
    mat_query: Query<&Handle<ColorMaterial>, With<ZoetropeImage>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    server: Res<AssetServer>,
) {
    let mat = mat_query.single();
    if let Some(material) = materials.get_mut(&mat) {
        material.texture = Some(server.load("background.png"));
    }
}
