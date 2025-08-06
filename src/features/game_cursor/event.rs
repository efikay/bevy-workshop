use bevy::prelude::*;

use super::resource::GameCursorType;

#[derive(Event, Reflect)]
pub struct SetGameCursor(pub Option<GameCursorType>);
