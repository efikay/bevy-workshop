use bevy::prelude::*;

pub enum TakeDamageResult {
    Dead,
    // So that's... progress
    StillAlive,
}

#[derive(Component)]
pub struct Health {
    hp: u32,
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
}
impl Health {
    pub fn new(hp: u32) -> Self {
        Self { hp }
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
