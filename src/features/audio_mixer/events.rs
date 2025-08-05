use bevy::prelude::*;

use super::resource::Stem;

#[derive(Event)]
pub struct LoadStemRequest {
    pub stem: Stem,
    pub path: String,
}

#[derive(Event)]
pub struct PausePlaybackRequest;

#[derive(Event)]
pub struct StopPlaybackRequest;

#[derive(Event)]
pub struct MuteStemRequest(pub Stem);

#[derive(Event)]
pub struct UnmuteStemRequest(pub Stem);

#[derive(Event)]
pub struct StartPlaybackRequest;
