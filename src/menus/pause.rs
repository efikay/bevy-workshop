//! The pause menu.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{menus::MenuState, screens::ScreenState, ui_kit::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(MenuState::Pause), spawn_pause_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(MenuState::Pause).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Pause Menu"),
        GlobalZIndex(2),
        StateScoped(MenuState::Pause),
        children![
            widget::header("Game paused"),
            widget::button("Continue", close_menu),
            widget::button("Settings", open_settings_menu),
            widget::button("Quit to title", quit_to_title),
        ],
    ));
}

fn open_settings_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<MenuState>>) {
    next_menu.set(MenuState::Settings);
}

fn close_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<MenuState>>) {
    next_menu.set(MenuState::None);
}

fn quit_to_title(_: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<ScreenState>>) {
    next_screen.set(ScreenState::Title);
}

fn go_back(mut next_menu: ResMut<NextState<MenuState>>) {
    next_menu.set(MenuState::None);
}
