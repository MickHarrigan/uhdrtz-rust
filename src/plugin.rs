use bevy::prelude::*;

use crate::systems::uhhhh;

pub struct ZoetropePlugin;

impl Plugin for ZoetropePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(uhhhh);
    }
}
