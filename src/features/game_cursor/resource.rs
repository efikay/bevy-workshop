use bevy::{prelude::*, winit::cursor::CustomCursor};
use hashbrown::HashMap;
use strum_macros::EnumIter;

#[derive(EnumIter, Reflect, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub enum GameCursorType {
    #[default]
    Pointer,
    PointerHovered,
    AimRelaxed,
    AimLocked,
}
impl GameCursorType {
    fn to_full_path(name: &'static str) -> String {
        // Retina 144x144
        const CURSORS_FOLDER: &'static str = "cursors/Kenney Crosshair Pack/PNG/Outline Retina";

        format!("{}/{}", CURSORS_FOLDER, name)
    }
    const fn to_hotpoint() -> &'static (u16, u16) {
        &(77, 77)
    }

    pub(super) fn asset_path(&self) -> String {
        match self {
            GameCursorType::Pointer => Self::to_full_path("crosshair001.png"),
            GameCursorType::PointerHovered => Self::to_full_path("crosshair008.png"),
            GameCursorType::AimRelaxed => Self::to_full_path("crosshair039.png"),
            GameCursorType::AimLocked => Self::to_full_path("crosshair040.png"),
        }
    }

    pub(super) const fn hotspot(&self) -> &'static (u16, u16) {
        Self::to_hotpoint()
    }
}

#[derive(Resource)]
pub struct GameCursorIcons(pub HashMap<GameCursorType, CustomCursor>);
