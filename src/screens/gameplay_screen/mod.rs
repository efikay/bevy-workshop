use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{menus::MenuState, scenes::player_scene::level, screens::ScreenState};
use systems::*;

mod systems;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(ScreenState::Gameplay), level::spawn_level);

    // Toggle pause on key press.
    app.add_systems(
        Update,
        (
            (pause, spawn_pause_overlay, open_pause_menu).run_if(
                in_state(ScreenState::Gameplay)
                    .and(in_state(MenuState::None))
                    .and(input_just_pressed(KeyCode::KeyP).or(input_just_pressed(KeyCode::Escape))),
            ),
            close_menu.run_if(
                in_state(ScreenState::Gameplay)
                    .and(not(in_state(MenuState::None)))
                    .and(input_just_pressed(KeyCode::KeyP)),
            ),
        ),
    );
    app.add_systems(OnExit(ScreenState::Gameplay), (close_menu, unpause));
    app.add_systems(
        OnEnter(MenuState::None),
        unpause.run_if(in_state(ScreenState::Gameplay)),
    );
}
