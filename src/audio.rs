use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub fn audio_setup(server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(server.load("RomanceAnonimo.mp3"))
        .looped()
        .with_volume(0.1);
}

pub fn audio_modulation_keyboard(input: Res<Input<KeyCode>>, audio: Res<Audio>) {
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
