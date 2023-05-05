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
    let mut rate: f64 = 1.0;
    if input.pressed(KeyCode::LShift) || input.pressed(KeyCode::RShift) {
        rate = 2.5;
    }
    if input.pressed(KeyCode::Left) {
        audio.set_playback_rate(-rate);
    } else if input.pressed(KeyCode::Right) {
        audio.set_playback_rate(rate);
    } else {
        audio.set_playback_rate(0.);
    }
}
