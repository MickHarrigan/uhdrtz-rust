use std::f32::consts::PI;

use crate::bluetooth::RotationInterval;
use crate::camera::{VideoFrame, VideoStream};
use crate::gui::{CameraCrosshairTag, CameraMaskTag, FULL, LOW, MED};
use crate::setup::Settings;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use nokhwa::pixel_format::RgbAFormat;
use nokhwa::utils::{CameraFormat, FrameFormat, RequestedFormat, RequestedFormatType, Resolution};

#[derive(Component)]
pub struct ZoetropeImage;

#[derive(Resource)]
pub struct ZoetropeMaxInterval(pub i8);

#[derive(Resource)]
pub struct Counter(pub u8);

pub fn zoetrope_setup(
    mut commands: Commands,
    // video_images: Res<VideoFrame>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<Settings>,
    server: Res<AssetServer>,
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

    commands
        .spawn(Camera2dBundle {
            transform: Transform::from_xyz(0., 0., 100.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(cam);

    commands
        .spawn(bevy::sprite::MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(800.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_xyz(0., 0., 50.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(ZoetropeImage);

    commands
        .spawn(SpriteBundle {
            texture: server.load("mask_full.png"),
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            visibility: Visibility::Hidden,
            ..default()
        })
        .insert(CameraMaskTag(FULL));
    commands
        .spawn(SpriteBundle {
            texture: server.load("mask1080.png"),
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            visibility: Visibility::Hidden,
            ..default()
        })
        .insert(CameraMaskTag(LOW));
    commands
        .spawn(SpriteBundle {
            texture: server.load("mask1440.png"),
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            visibility: Visibility::Hidden,
            ..default()
        })
        .insert(CameraMaskTag(MED));
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
    mut query: Query<&mut Transform, Or<(With<ZoetropeImage>, With<CameraMaskTag>)>>,
    rotation: Res<RotationInterval>,
    max: Res<ZoetropeMaxInterval>,
) {
    // the ratio between 24 fps and 60 fps is 0.4
    // this means that every 2.5 times the animation should run
    // here this is making it 3, though 2 could be used instead potentially.
    // This breaks down to either the ceil() or floor() functions on the
    // ratio output.
    // if count.0 == 3 {
    //     count.0 = 0;
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
        transform.rotate_z(PI / 12.0 * val);
    }
    // } else {
    //     count.0 += 1;
    // }
}

pub fn zoetrope_animation_keyboard(
    // mut query: Query<&mut Transform, With<ZoetropeImage>>,
    mut query: Query<&mut Transform, Or<(With<ZoetropeImage>, With<CameraMaskTag>)>>,
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
