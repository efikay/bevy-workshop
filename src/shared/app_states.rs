use bevy::{reflect::Reflect, state::state::States};

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Reflect)]
#[states(scoped_entities)]
pub struct AppPauseState(pub bool);
