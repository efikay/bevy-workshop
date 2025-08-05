#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct Creature {
    pub is_controlled: bool,
}
