mod events;
mod resource;
mod systems;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_seedling::SeedlingPlugin;

pub fn plugin(app: &mut App) {
    app.add_plugins(SeedlingPlugin::default());

    app.init_resource::<resource::AudioMixer>();
    app.register_type::<resource::AudioMixer>();

    #[cfg(feature = "inspector__audio_mixer")]
    {
        use bevy_inspector_egui::quick::{FilterQueryInspectorPlugin, ResourceInspectorPlugin};
        use bevy_seedling::sample::SamplePlayer;

        app.add_plugins(ResourceInspectorPlugin::<resource::AudioMixer>::default());
        app.add_plugins(FilterQueryInspectorPlugin::<With<SamplePlayer>>::default());
    }

    app.add_event::<events::LoadStemRequest>();
    app.add_event::<events::PausePlaybackRequest>();
    app.add_event::<events::StopPlaybackRequest>();
    app.add_event::<events::AudioPlayRequestEvent>();
    app.add_event::<events::MuteStemRequest>();
    app.add_event::<events::UnmuteStemRequest>();

    app.add_systems(Startup, systems::initialize_audio);
    app.add_systems(
        Update,
        (
            systems::load_requested_audio,
            systems::pause_tracks,
            systems::stop_tracks,
            systems::play_tracks,
            systems::mute_requested_track,
            systems::unmute_requested_track,
        ),
    );

    app.add_systems(
        Startup,
        (systems::debug::add_some_music, systems::debug::play_music),
    );

    app.add_systems(
        Update,
        (
            systems::debug::add_some_music.run_if(input_just_pressed(KeyCode::KeyA)),
            systems::debug::pause_music.run_if(input_just_pressed(KeyCode::KeyP)),
            systems::debug::stop_music.run_if(input_just_pressed(KeyCode::KeyS)),
            systems::debug::play_music.run_if(input_just_pressed(KeyCode::KeyK)),
            systems::debug::mute_all_but_drums.run_if(input_just_pressed(KeyCode::KeyM)),
            systems::debug::unmute_all.run_if(input_just_pressed(KeyCode::KeyU)),
        ),
    );
}
