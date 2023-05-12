use crate::bluetooth::RotationInterval;
use crate::setup::Settings;
use crate::zoetrope::{RotationDirection, ZoetropeAnimationThresholdSpeed};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

#[derive(Resource)]
pub struct Song(pub String);

pub struct VolumeEvent(pub f64);

pub fn audio_setup(server: Res<AssetServer>, audio: Res<Audio>, settings: Res<Settings>) {
    match &settings.song {
        Some(music) => {
            audio
                .play(server.load(format!("audio/{}", music)))
                .looped()
                .with_volume(0.5);
        }
        None => {}
    }
}

#[allow(dead_code)]
pub fn audio_modulation_keyboard(input: Res<Input<KeyCode>>, audio: Res<Audio>) {
    let mut rate: f64 = 1.0;
    if input.pressed(KeyCode::LShift) || input.pressed(KeyCode::RShift) {
        rate = 2.5;
    }
    if input.pressed(KeyCode::Left) {
        audio.set_playback_rate(rate);
    } else if input.pressed(KeyCode::Right) {
        audio.set_playback_rate(-rate);
    } else {
        audio.set_playback_rate(0.);
    }
}

pub fn audio_modulation_rotation(
    rotation: Res<RotationInterval>,
    max: Res<ZoetropeAnimationThresholdSpeed>,
    audio: Res<Audio>,
    dir: Res<RotationDirection>,
) {
    let val: f64;
    if rotation.0 >= max.0 {
        val = (!dir.audio * 1.0) as f64;
    } else if rotation.0 <= -max.0 {
        val = (dir.audio * 1.0) as f64;
    } else {
        val = rotation.0 as f64 / max.0 as f64;
    }
    audio
        .set_playback_rate(val)
        .linear_fade_in(std::time::Duration::from_secs_f64(1.0 / max.0 as f64));
}

pub fn change_audio_volume(audio: Res<Audio>, mut vol_change: EventReader<VolumeEvent>) {
    for ev in vol_change.iter() {
        audio.set_volume(ev.0);
    }

    vol_change.clear();
}
