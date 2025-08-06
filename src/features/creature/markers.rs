#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct Creature;

#[derive(Component, Reflect)]
pub struct ControlledCreature;

// Creature type (one of):
#[derive(Component, Reflect)]
pub struct Player;

#[derive(Component, Reflect)]
pub struct EnemyNPC;

#[derive(Component, Reflect)]
pub struct FriendNPC;

#[derive(Component, Reflect)]
pub struct NeutralNPC;
