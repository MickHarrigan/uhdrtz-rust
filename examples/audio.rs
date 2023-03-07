use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_startup_system(background_audio)
        .add_system(get_input)
        .run()
}

fn background_audio(server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(server.load("RomanceAnonimo.mp3"))
        .looped()
        .with_volume(0.1);
}

fn get_input(input: Res<Input<KeyCode>>, audio: Res<Audio>) {
    // push the song forwards or backwards based on the direction pressed with the arrow keys

    if input.pressed(KeyCode::Left) {
        audio.set_playback_rate(-1.0);
    } else if input.pressed(KeyCode::Right) {
        audio.set_playback_rate(2.0);
    } else {
        audio.set_playback_rate(1.0);
    }
}
