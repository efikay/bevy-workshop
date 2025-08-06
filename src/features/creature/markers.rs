#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct Creature;

// ================== Creature type (one of): ================== //
pub mod creature_type {
    use super::*;

    #[derive(Component, Reflect)]
    pub struct Player;

    #[derive(Component, Reflect)]
    pub struct EnemyNPC;

    #[derive(Component, Reflect)]
    pub struct FriendNPC;

    #[derive(Component, Reflect)]
    pub struct NeutralNPC;
}
