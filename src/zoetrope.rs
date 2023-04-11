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

pub fn zoetrope_setup(
    mut commands: Commands,
    // video_images: Res<VideoFrame>,
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
        .spawn(SpriteBundle {
            texture: Handle::default(),
            transform: Transform::from_xyz(0.0, 0.0, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
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
    time: Res<Time>,
    mut query: Query<&mut Transform, Or<(With<ZoetropeImage>, With<CameraMaskTag>)>>,
    rotation: Res<RotationInterval>,
    max: Res<ZoetropeMaxInterval>,
) {
    for mut transform in query.iter_mut() {
        // https://github.com/bevyengine/bevy/blob/main/examples/2d/rotation.rs
        let val: f32;
        // rotation is an i8
        // need to get it to an f32
        if rotation.0 > max.0 {
            val = (max.0).into();
        } else if rotation.0 < -max.0 {
            val = (-max.0).into();
        } else {
            val = (rotation.0).into();
        }
        transform.rotate_z(time.delta_seconds() * val /*rotation.0 as f32*/);
    }
}

pub fn zoetrope_next_camera_frame(
    cam_query: Query<&mut VideoStream>,
    // image: Res<VideoFrame>,
    // settings: Res<Settings>,
    mut images: ResMut<Assets<Image>>,
    mut tex_query: Query<&mut Handle<Image>, With<ZoetropeImage>>,
) {
    let camera = cam_query.single();
    // let wh = (settings.resolution.width(), settings.resolution.height());
    if let Some(image) = camera.image_rx.drain().last() {
        // *tex_query.single_mut() = images.add(image);
        let mut tex = tex_query.single_mut();
        *tex = images.set(tex.clone_weak(), image);
    }
}
