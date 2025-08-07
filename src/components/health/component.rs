use bevy::prelude::*;

pub enum TakeDamageResult {
    Dead,
    // So that's... progress
    StillAlive,
}

#[derive(Component, Reflect)]
#[require(Sprite, Transform)]
pub struct Health {
    hp: u32,
    max_hp: u32,
}
impl Health {
    #[inline]
    pub fn hp(&self) -> u32 {
        self.hp
    }
    #[inline]
    pub fn is_dead(&self) -> bool {
        self.hp == 0
    }
    #[inline]
    pub fn percent_ratio(&self) -> f32 {
        if self.max_hp == 0 || self.hp == 0 {
            0.0
        } else {
            self.hp as f32 / self.max_hp as f32
        }
    }
}
impl Health {
    pub fn new(hp: u32) -> Self {
        Self { hp, max_hp: hp }
    }

    pub fn take_damage(&mut self, damage: u32) -> TakeDamageResult {
        self.hp = self.hp.checked_sub(damage).unwrap_or(0);

        if self.is_dead() {
            TakeDamageResult::Dead
        } else {
            TakeDamageResult::StillAlive
        }
    }
}
