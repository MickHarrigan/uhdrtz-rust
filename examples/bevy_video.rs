use bevy::prelude::*;
use uhdrtz::prelude::*;

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
