use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksPlugin;

use crate::bluetooth::{async_spawner, ZoetropeRotation};
use crate::camera::VideoFrame;
use crate::zoetrope::{logical_camera_rotation, update_zoetrope_image, zoetrope_setup};

pub struct ZoetropePlugin;

impl Plugin for ZoetropePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VideoFrame(Handle::default()))
            .insert_resource(ClearColor(Color::BLACK))
            .add_plugin(TokioTasksPlugin::default())
            .insert_resource(ZoetropeRotation(0))
            .add_startup_system(zoetrope_setup)
            .add_startup_system(async_spawner)
            .add_system(bevy::window::close_on_esc)
            .add_system(update_zoetrope_image)
            .add_system(logical_camera_rotation);
    }
}
