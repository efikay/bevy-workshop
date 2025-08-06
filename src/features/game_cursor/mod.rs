#![allow(dead_code)]

use bevy::prelude::*;

pub mod event;
mod resource;
mod system;

pub fn plugin(app: &mut App) {
    app.add_event::<event::SetGameCursor>();
    app.register_type::<event::SetGameCursor>();

    app.add_systems(Startup, system::init_cursor_icons);
    app.add_systems(Update, system::update_cursor_from_event);

    // app.add_plugins(debug::plugin);
}

mod debug {
    use crate::features::game_cursor::resource::GameCursorType;

    use super::*;
    use bevy::input::common_conditions::input_just_pressed;
    use strum::IntoEnumIterator;

    pub fn plugin(app: &mut App) {
        app.add_systems(
            Update,
            cycle_cursors.run_if(input_just_pressed(KeyCode::KeyC)),
        );
        app.add_systems(
            Update,
            toggle_cursor_visibility.run_if(input_just_pressed(KeyCode::KeyV)),
        );
    }

    fn toggle_cursor_visibility(
        mut writer: EventWriter<event::SetGameCursor>,
        mut visible: Local<bool>,
    ) {
        *visible = !*visible;

        writer.write(event::SetGameCursor(
            (*visible).then_some(GameCursorType::default()),
        ));
    }

    fn cycle_cursors(
        mut writer: EventWriter<event::SetGameCursor>,
        mut cursor: Local<resource::GameCursorType>,
    ) {
        writer.write(event::SetGameCursor(Some(cursor.to_owned())));

        // Searching for next cursor
        let mut cursors = resource::GameCursorType::iter().cycle().peekable();
        loop {
            let it = cursors.next().unwrap();

            if it == *cursor {
                *cursor = *cursors.peek().unwrap();

                break;
            }
        }
    }
}
