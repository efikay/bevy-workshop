use bevy::{
    prelude::*,
    winit::cursor::{CursorIcon, CustomCursor, CustomCursorImage},
};
use hashbrown::HashMap;
use strum::IntoEnumIterator;

use super::event::SetGameCursor;
use super::resource::{GameCursorIcons, GameCursorType};

pub fn init_cursor_icons(mut commands: Commands, asset_server: Res<AssetServer>) {
    let cursor_icons = HashMap::from_iter(GameCursorType::iter().map(|cursor| {
        (
            cursor,
            CustomCursor::Image(CustomCursorImage {
                handle: asset_server.load(cursor.asset_path()),
                hotspot: *cursor.hotspot(),
                ..default()
            }),
        )
    }));

    commands.insert_resource(GameCursorIcons(cursor_icons));
}

pub fn update_cursor_from_event(
    mut commands: Commands,
    mut window: Single<(Entity, &mut Window), With<Window>>,
    mut events: EventReader<SetGameCursor>,
    cursors: Res<GameCursorIcons>,
) {
    for SetGameCursor(requested_cursor) in events.read() {
        window.1.cursor_options.visible = requested_cursor.is_some();
        println!("Hey! Look! {}", window.1.cursor_options.visible);

        let Some(requested_cursor_type) = requested_cursor else {
            return;
        };

        let desired_cursor = cursors.0.get(requested_cursor_type).unwrap();

        commands
            .entity(window.0)
            .insert(CursorIcon::Custom(desired_cursor.clone()));
    }
}
