use bevy::prelude::*;

use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{
    CameraFormat, CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType, Resolution,
};
use nokhwa::Buffer;
use nokhwa::Camera as PhysicalCamera;

#[derive(Component)]
struct VideoStream;

impl VideoStream {
    pub fn new(index: u32, format: RequestedFormat) -> Image {
        // create a new camera connection a la the nokhwa example
        let mut camera = PhysicalCamera::new(CameraIndex::Index(index), format)
            .expect("Could not create a physical camera connection");

        camera
            .open_stream()
            .expect("Could not open the camera stream");
        todo!();
    }
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
        .add_startup_system(setup)
        .add_system(camera_rotation) // function that rotates the camera automatically, will update to be based on input next
        .run()
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>, assets: Res<AssetServer>) {
    // next up is to open a camera (both physical camera for taking an image as well as the logical bevy one that looks at a plane)
    // then open a stream from the camera with the right settings
    // then constantly (read: every frame of the "game") get and image from the camera
    // and to display that image to a plane that a 2d camera is looking at

    // TODO: this should be filled in from the nokhwa stuff
    // let video_output: Handle<Image> = images.add(Image::default());
    let video_output = assets.load("image.png");

    // TODO: this camera could be tweaked to change where and what it is looking at
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(SpriteBundle {
        // the clone() could be redundant, so will have to check that in the coming time
        texture: video_output.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y), // TODO: update the transform
        ..default()
    });
}

fn camera_rotation(time: Res<Time>, mut query: Query<&mut Transform, With<Camera>>) {
    // let sec = time.elapsed_seconds() * 0.2;
    for mut transform in query.iter_mut() {
        // *transform = Transform::from_xyz(sec.sin() * 5.0, 2.5, sec.cos() * 5.0)
        transform.rotate_z(time.delta_seconds());
    }
}
