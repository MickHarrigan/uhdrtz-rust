use bevy::prelude::*;

use crate::components::VideoFrame;
use crate::systems::{camera_rotation, handle_video_frame, setup_physical_camera};

pub struct ZoetropePlugin;

impl Plugin for ZoetropePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VideoFrame(Handle::default()))
            .add_startup_system(setup_physical_camera)
            .add_system(handle_video_frame)
            .add_system(camera_rotation);
    }
}
