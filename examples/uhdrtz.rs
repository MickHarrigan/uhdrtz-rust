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
                    window: WindowDescriptor {
                        mode: WindowMode::BorderlessFullscreen,
                        present_mode: PresentMode::Fifo,
                        ..default()
                    },
                    ..default()
                }),
        )
        .add_plugin(ZoetropePlugin)
        .run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostics() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugin(ZoetropePlugin)
            .add_startup_system(simple_checks_startup)
            .run();
    }

    fn simple_checks_startup(mut image: ResMut<VideoFrame>, mut images: ResMut<Assets<Image>>) {
        println!("{:?}", image.0);
        // this below returns None
        println!("{:?}", images.get(&image.0));
        // this changes the handle to a strong, but is conveniently weak as soon as this ends
        println!("{:?}", images.set(&image.0, Image::default()));
        // that is why adding this should then update the handle across the board
        // SUCCESS: this did work
        image.0 = images.set(&image.0, Image::default());
        println!("{:?}", images.get(&image.0));
    }
}
