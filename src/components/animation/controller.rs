#![allow(dead_code)]

use std::time::Duration;

use bevy::prelude::*;

use crate::shared::data_structures::IterableRangeGrid;
use crate::shared::direction::DirectionSimple;

use super::animation_state::AnimationState;

#[derive(Component, Debug)]
pub struct AnimationController {
    timer: Timer,
    frame_grid: IterableRangeGrid<AnimationState, DirectionSimple>,
}
impl AnimationController {
    // TODO: Move from constant to struct field
    // TODO: Make configurable for each animation
    const ANIMATION_INTERVAL: Duration = Duration::from_millis(200);
    // TODO: Move from constant to struct field
    const KEEP_DIRECTION_ON_IDLE: bool = true;

    pub fn new(frame_grid: IterableRangeGrid<AnimationState, DirectionSimple>) -> Self {
        Self {
            timer: Timer::new(Self::ANIMATION_INTERVAL, TimerMode::Repeating),
            frame_grid,
        }
    }

    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);

        if !self.timer.finished() {
            return;
        }

        self.frame_grid.to_next();
    }

    #[inline]
    pub fn changed(&self) -> bool {
        self.timer.finished()
    }

    #[inline]
    pub fn frame(&self) -> u8 {
        self.frame_grid.cursor()
    }

    pub fn update_from_point(&mut self, point: Vec2) {
        let animation_state = AnimationState::from(point);
        self.frame_grid.update_outer(animation_state);

        // TODO: More readable comparison (maybe split to #[inline] function or smth)
        // If need to preserve idle and we're idle - no need to update direction (we're preserving, remember?)
        // If no need to preserve – we just update in any case (but if we're idle we'll reset to default direction because idle means no speed what means no direction)
        if !Self::KEEP_DIRECTION_ON_IDLE
            || (Self::KEEP_DIRECTION_ON_IDLE && animation_state != AnimationState::Idle)
        {
            let direction = DirectionSimple::from(point);
            self.frame_grid.update_inner(direction);
        }
    }
}
