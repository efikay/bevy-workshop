use bevy::prelude::*;

use super::*;

pub fn bundle() -> impl Bundle {
    (marker::GameCam, Camera2d::default())
}
