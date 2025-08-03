use bevy::{prelude::*, ui::Val::*};

use crate::{menus::MenuState, shared::AppPauseState};

pub(super) fn unpause(mut next_pause: ResMut<NextState<AppPauseState>>) {
    next_pause.set(AppPauseState(false));
}

pub(super) fn pause(mut next_pause: ResMut<NextState<AppPauseState>>) {
    next_pause.set(AppPauseState(true));
}

pub(super) fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        Name::new("Pause Overlay"),
        Node {
            width: Percent(100.0),
            height: Percent(100.0),
            ..default()
        },
        GlobalZIndex(1),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        StateScoped(AppPauseState(true)),
    ));
}

pub(super) fn open_pause_menu(mut next_menu: ResMut<NextState<MenuState>>) {
    next_menu.set(MenuState::Pause);
}

pub(super) fn close_menu(mut next_menu: ResMut<NextState<MenuState>>) {
    next_menu.set(MenuState::None);
}
