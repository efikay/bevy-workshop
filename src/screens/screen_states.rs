use bevy::state::state::States;

/// The game's main screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum ScreenState {
    Splash,
    Title,
    Loading,
    #[default]
    Gameplay,
}
