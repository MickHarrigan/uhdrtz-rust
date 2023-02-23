use crate::bluetooth::ZoetropeRotation;
use crate::camera::{VideoFrame, VideoStream};
use crate::gui::{CrossImage, MaskImage, FULL, HALF};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use nokhwa::pixel_format::RgbAFormat;
use nokhwa::utils::{CameraFormat, FrameFormat, RequestedFormat, RequestedFormatType, Resolution};

#[derive(Component)]
pub struct ZoetropeImage;

pub fn zoetrope_setup(
    mut commands: Commands,
    video_images: Res<VideoFrame>,
    server: Res<AssetServer>,
) {
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
            transform: Transform::from_xyz(0., 0., 100.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(cam);

    commands
        .spawn(SpriteBundle {
            texture: video_images.0.clone_weak(),
            transform: Transform::from_xyz(0.0, 0.0, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(ZoetropeImage);

    commands
        .spawn(SpriteBundle {
            texture: server.load("mask_full.png"),
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            visibility: Visibility::INVISIBLE,
            ..default()
        })
        .insert(MaskImage(FULL));
    commands
        .spawn(SpriteBundle {
            texture: server.load("mask_half.png"),
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            visibility: Visibility::INVISIBLE,
            ..default()
        })
        .insert(MaskImage(HALF));
    commands
        .spawn(SpriteBundle {
            texture: server.load("xhair.png"),
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            visibility: Visibility::INVISIBLE,
            ..default()
        })
        .insert(CrossImage);
}

pub fn logical_camera_rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera>>,
    rotation: Res<ZoetropeRotation>,
) {
    for mut transform in query.iter_mut() {
        // https://github.com/bevyengine/bevy/blob/main/examples/2d/rotation.rs
        transform.rotate_z(time.delta_seconds() * rotation.0 as f32);
    }
}

pub fn logical_camera_movement(mut query: Query<&mut Transform, With<Camera>>, x_pos, y_pos, zoom) {
    
}

pub fn update_zoetrope_image(
    cam_query: Query<&mut VideoStream>,
    image: Res<VideoFrame>,
    mut images: ResMut<Assets<Image>>,
    mut tex_query: Query<&mut Handle<Image>, With<ZoetropeImage>>,
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
