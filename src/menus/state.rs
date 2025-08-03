use bevy::state::state::States;

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum MenuState {
    #[default]
    None,
    Main,
    Credits,
    Settings,
    Pause,
}
