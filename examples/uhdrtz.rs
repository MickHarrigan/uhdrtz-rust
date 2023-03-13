// this is the actual testing ground that is the primary test.
// This will also be what the final binary should be made of and deployed from.
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use uhdrtz::prelude::*;

fn main() {
    App::new()
        // this plugin stuff here could be set into a large Zoetrope plugin that is controlled in the library itself
        // thus that the actual example is just adding in that one plugin to the bevy system and its done
        .add_plugins(
            DefaultPlugins
                .build()
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: bevy::window::WindowMode::BorderlessFullscreen,
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                }),
            //     {
            //     window: WindowDescriptor {
            //         mode: WindowMode::BorderlessFullscreen,
            //         present_mode: PresentMode::Fifo,
            //         ..default()
            //     },
            //     ..default()
            // }),
        )
        .add_plugins(ZoetropePlugins)
        .run()
}
