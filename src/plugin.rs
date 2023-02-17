use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksPlugin;

use crate::bluetooth::{async_spawner, ZoetropeRotation};
use crate::camera::VideoFrame;
use crate::systems::{logical_camera_rotation, physical_camera_setup, update_zoetrope_image};

pub struct ZoetropePlugin;

impl Plugin for ZoetropePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VideoFrame(Handle::default()))
            .add_plugin(TokioTasksPlugin::default())
            .insert_resource(ZoetropeRotation(0))
            .add_startup_system(physical_camera_setup)
            .add_startup_system(async_spawner)
            .add_system(bevy::window::close_on_esc)
            .add_system(update_zoetrope_image)
            .add_system(logical_camera_rotation);
    }
}
