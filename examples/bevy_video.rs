use bevy::prelude::*;

use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{
    CameraFormat, CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType, Resolution,
};
use nokhwa::{Buffer, Camera};

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
        .run()
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // next up is to open a camera (both physical camera for taking an image as well as the logical bevy one that looks at a plane)
    // then open a stream from the camera with the right settings
    // then constantly (read: every frame of the "game") get and image from the camera
    // and to display that image to a plane that a 2d camera is looking at

    // TODO: this should be filled in from the nokhwa stuff
    let video_output: Handle<Image> = images.add(Image::default());

    // TODO: this camera could be tweaked to change where and what it is looking at
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        // the clone() could be redundant, so will have to check that in the coming time
        texture: video_output.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0), // TODO: update the transform
        ..default()
    });
}
