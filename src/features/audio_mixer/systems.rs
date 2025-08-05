use bevy::prelude::*;
use bevy_seedling::prelude::*;

use bevy_seedling::sample::{PlaybackSettings, Sample};

use crate::features::audio_mixer::resource::{AudioMixer, Stem};

use super::events::{
    StartPlaybackRequest, LoadStemRequest, MuteStemRequest, PausePlaybackRequest,
    StopPlaybackRequest, UnmuteStemRequest,
};

pub fn initialize_audio(mut master: Single<&mut VolumeNode, With<MainBus>>) {
    // Since the main bus already exists, we can just set the desired volume.
    master.volume = Volume::UNITY_GAIN;
}

pub fn load_requested_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut audio_requests: EventReader<LoadStemRequest>,
    mut mixer: ResMut<AudioMixer>,
) {
    for LoadStemRequest { path, stem } in audio_requests.read() {
        let audio = asset_server.load::<Sample>(path.to_owned());

        if let Ok(()) = mixer.register_stem(audio.clone(), stem) {
            commands.spawn((
                SamplerPool(stem.clone()),
                SamplePlayer::new(audio),
                VolumeNode {
                    volume: Volume::Linear(1.0),
                },
                PlaybackSettings {
                    playback: Notify::new(PlaybackState::Stop),
                    playhead: Notify::default(),
                    speed: 1.0,
                    on_complete: OnComplete::Preserve,
                },
            ));
        }
    }
}

pub fn pause_tracks(
    mut tracks: Query<&mut PlaybackSettings>,
    mut pause_requests: EventReader<PausePlaybackRequest>,
) {
    for _ in pause_requests.read() {
        for mut playback in &mut tracks {
            playback.pause();
        }
    }
}

pub fn stop_tracks(
    mut tracks: Query<&mut PlaybackSettings>,
    mut stop_requests: EventReader<StopPlaybackRequest>,
) {
    for _ in stop_requests.read() {
        for mut playback in &mut tracks {
            playback.stop();
        }
    }
}

pub fn play_tracks(
    mut tracks: Query<&mut PlaybackSettings>,
    mut play_requests: EventReader<StartPlaybackRequest>,
) {
    for _ in play_requests.read() {
        for mut playback in &mut tracks {
            playback.play();
        }
    }
}

pub fn mute_requested_track(
    mut tracks_q: Query<(&SamplerPool<Stem>, &mut VolumeNode)>,
    mut mute_requests: EventReader<MuteStemRequest>,
) {
    for requested_stem in mute_requests.read() {
        for (stem, mut volume) in &mut tracks_q {
            if stem.0 == requested_stem.0 {
                volume.volume = Volume::Linear(0.0);
            }
        }
    }
}

pub fn unmute_requested_track(
    mut tracks_q: Query<(&SamplerPool<Stem>, &mut VolumeNode)>,
    mut mute_requests: EventReader<UnmuteStemRequest>,
) {
    for requested_stem in mute_requests.read() {
        for (stem, mut volume) in &mut tracks_q {
            if stem.0 == requested_stem.0 {
                volume.volume = Volume::Linear(1.0);
            }
        }
    }
}

pub mod debug {
    use crate::features::audio_mixer::{
        events::{
            StartPlaybackRequest, LoadStemRequest, MuteStemRequest, PausePlaybackRequest,
            StopPlaybackRequest, UnmuteStemRequest,
        },
        resource::Stem,
    };
    use bevy::{log, prelude::*};

    const BASS: &'static str =
        "audio/es/music/Lost in You (Instrumental Version) - DJ Mayson/bass.mp3";
    const DRUMS: &'static str =
        "audio/es/music/Lost in You (Instrumental Version) - DJ Mayson/drums.mp3";
    const INSTRUMENTS: &'static str =
        "audio/es/music/Lost in You (Instrumental Version) - DJ Mayson/instruments.mp3";

    pub fn add_some_music(mut writer: EventWriter<LoadStemRequest>) {
        log::info!("debug:: Add bass+instruments+drums request sent!");

        writer.write_batch(vec![
            LoadStemRequest {
                path: BASS.into(),
                stem: Stem::Bass,
            },
            LoadStemRequest {
                path: DRUMS.into(),
                stem: Stem::Drums,
            },
            LoadStemRequest {
                path: INSTRUMENTS.into(),
                stem: Stem::Instruments,
            },
        ]);
    }

    pub fn play_music(mut writer: EventWriter<StartPlaybackRequest>) {
        log::info!("debug:: Play music request sent!");

        writer.write(StartPlaybackRequest);
    }
    pub fn pause_music(mut writer: EventWriter<PausePlaybackRequest>) {
        log::info!("debug:: Pause music request sent!");

        writer.write(PausePlaybackRequest);
    }
    pub fn stop_music(mut writer: EventWriter<StopPlaybackRequest>) {
        log::info!("debug:: Stop music request sent!");

        writer.write(StopPlaybackRequest);
    }

    pub fn mute_all_but_drums(mut writer: EventWriter<MuteStemRequest>) {
        log::info!("debug:: Mute bass+instruments request sent!");

        writer.write_batch(vec![
            MuteStemRequest(Stem::Bass),
            MuteStemRequest(Stem::Instruments),
        ]);
    }
    pub fn unmute_all(mut writer: EventWriter<UnmuteStemRequest>) {
        log::info!("debug:: Unmute bass+instruments+drums request sent!");

        writer.write_batch(vec![
            UnmuteStemRequest(Stem::Bass),
            UnmuteStemRequest(Stem::Instruments),
            UnmuteStemRequest(Stem::Drums),
        ]);
    }
}
