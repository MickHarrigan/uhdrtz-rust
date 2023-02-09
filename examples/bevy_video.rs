use anyhow::Result;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::texture::{CompressedImageFormats, ImageType};
use flume::unbounded;

use image::{DynamicImage, RgbaImage};
use nokhwa::pixel_format::RgbAFormat;
use nokhwa::utils::{
    CameraFormat, CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType, Resolution,
};
use nokhwa::CallbackCamera;

#[derive(Component)]
struct VideoStream {
    pub image_rx: flume::Receiver<RgbaImage>,
}

#[derive(Resource, Clone)]
struct VideoFrame(pub Handle<Image>);

impl VideoStream {
    pub fn new(index: u32, format: RequestedFormat) -> Result<Self> {
        // lots of this is *heavily* taken from https://github.com/foxzool/bevy_nokhwa/blob/main/src/camera.rs
        let (sender, receiver) = unbounded();

        let callback_fn = move |buffer: nokhwa::Buffer| {
            let image = buffer.decode_image::<RgbAFormat>().unwrap();
            let _ = sender.send(image);
        };

        let mut threaded_camera =
            CallbackCamera::new(CameraIndex::Index(index), format, callback_fn)
                .expect("Could not create a CallbackCamera");

        threaded_camera
            .open_stream()
            .expect("Could not open the camera stream");

        std::thread::spawn(move || {
            #[allow(clippy::empty_loop)]
            loop {
                threaded_camera
                    .last_frame()
                    .expect("Couldn't receive the latest frame");
            }
        });

        // return something here
        Ok(Self { image_rx: receiver })
    }
}

fn handle_video_frame(
    cam_query: Query<&mut VideoStream>,
    // texture_query: Query<&mut Handle<Image>>,
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
                // this shows that there are images within tex
                // println!("{:?}", images.get(&tex));
            }
            // image.0 = img;
            // instead of setting a resource, try querying for the image handle itself to change
            // let texture = images.add(Image::new_fill(
            //     Extent3d {
            //         width: 3840,
            //         height: 2160,
            //         depth_or_array_layers: 1,
            //     },
            //     TextureDimension::D2,
            //     &img,
            //     TextureFormat::Rgba8UnormSrgb,
            // ));
            // println!("{:?}", texture);
            // image.0 = images.set(
            //     &image.0,
            //     Image::new_fill(
            //         Extent3d {
            //             width: 3840,
            //             height: 2160,
            //             depth_or_array_layers: 1,
            //         },
            //         TextureDimension::D2,
            //         &img,
            //         TextureFormat::Rgba8UnormSrgb,
            //     ),
            // );
        }
    }
}

fn simple_checks_startup(
    cam_query: Query<&mut VideoStream>,
    // texture_query: Query<&mut Handle<Image>>,
    mut image: ResMut<VideoFrame>,
    mut images: ResMut<Assets<Image>>,
) {
    println!("{:?}", image.0);
    // this below returns None
    println!("{:?}", images.get(&image.0));
    // this changes the handle to a strong, but is conveniently weak as soon as this ends
    println!("{:?}", images.set(&image.0, Image::default()));
    // that is why adding this should then update the handle across the board
    // SUCCESS: this did work
    image.0 = images.set(&image.0, Image::default());
    println!("{:?}", images.get(&image.0));

    // for camera in cam_query.iter() {
    //     while let Some(img) = camera.image_rx.drain().last() {
    //     }
    // }
}

fn simple_checks(
    cam_query: Query<&mut VideoStream>,
    // texture_query: Query<&mut Handle<Image>>,
    image: ResMut<VideoFrame>,
    mut images: ResMut<Assets<Image>>,
    mut tex_query: Query<&mut Handle<Image>>,
) {
    // this should check the VideoFrame and VideoStream to make sure that they are functioning correctly
    for (cam, count) in cam_query.iter().zip(0..) {
        // this proves that there is only the single VideoStream
        // println!("Camera Found! {}", count);
    }
    // for mut tex in &mut tex_query.iter_mut() {
    //     *tex = image.0.clone();
    // }
    println!("{:?}", image.0);
}

fn main() {
    App::new()
        // this plugin stuff here could be set into a large Zoetrope plugin that is controlled in the library itself
        // thus that the actual example is just adding in that one plugin to the bevy system and its done
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            },
            ..default()
        }))
        // .insert_resource(VideoFrame(RgbaImage::new(3840, 2160)))
        .insert_resource(VideoFrame(Handle::default()))
        .add_startup_system(simple_checks_startup)
        .add_startup_system(setup_physical_camera)
        // .add_system(simple_checks)
        .add_system(handle_video_frame)
        .add_system(camera_rotation) // function that rotates the camera automatically, will update to be based on input next
        .run()
}

fn setup_physical_camera(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    video_images: Res<VideoFrame>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // assets: ResMut<AssetServer>,
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
            transform: Transform::from_xyz(0., 0., 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(cam);

    // TODO: this should be filled in from the nokhwa stuff
    // let video_output = assets.load("image.png");
    // let handle = images.add(bevy::render::texture::Image::from_dynamic(
    //     DynamicImage::ImageRgba8(video_images.0.clone()),
    //     true,
    // ));

    // NOTE: video_images.0 is all 0s for some reason
    // println!("{:?}", video_images.0);

    // let video_output = images.add(bevy::render::texture::Image::new_fill(
    //     Extent3d {
    //         width: 3840,
    //         height: 2160,
    //         depth_or_array_layers: 1,
    //     },
    //     TextureDimension::D2,
    //     &video_images.0,
    //     TextureFormat::Rgba8UnormSrgb,
    // ));

    commands.spawn(SpriteBundle {
        // the clone() could be redundant, so will have to check that in the coming time
        texture: video_images.0.clone_weak(),
        // texture: handle,
        transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y), // TODO: update the transform
        ..default()
    });
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
    //     material: materials.add(StandardMaterial {
    //         base_color_texture: Some(video_output.clone()),
    //         ..default()
    //     }),
    //     ..default()
    // });
}

fn camera_rotation(time: Res<Time>, mut query: Query<&mut Transform, With<Camera>>) {
    for mut transform in query.iter_mut() {
        // https://github.com/bevyengine/bevy/blob/main/examples/2d/rotation.rs
        transform.rotate_z(time.delta_seconds());
    }
}
