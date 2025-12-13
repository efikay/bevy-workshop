use std::time::Duration;

use bevy::prelude::*;
use lib::data_structures::range_double_mapper::RangeDoubleMapper;

use crate::shared::direction::DirectionAdvanced;

use super::animation_state::AnimationState;

#[derive(Component, Debug)]
pub struct Animation {
    timer: Timer,
    frame_grid: RangeDoubleMapper<AnimationState, DirectionAdvanced>,
}
impl Animation {
    // TODO: Make configurable for each animation
    const DEFAULT_ANIMATION_INTERVAL: Duration = Duration::from_millis(200);
    // TODO: Move from constant to struct field
    const KEEP_DIRECTION_ON_IDLE: bool = true;
}
impl Animation {
    pub fn new(frame_grid: RangeDoubleMapper<AnimationState, DirectionAdvanced>) -> Self {
        Self::new_with_animation_interval(frame_grid, Self::DEFAULT_ANIMATION_INTERVAL)
    }
    pub fn new_with_animation_interval(
        frame_grid: RangeDoubleMapper<AnimationState, DirectionAdvanced>,
        interval: Duration,
    ) -> Self {
        Self {
            frame_grid,
            timer: Timer::new(interval, TimerMode::Repeating),
        }
    }

    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);

        if !self.timer.finished() {
            return;
        }

        self.frame_grid.to_next();
    }

    pub fn idle_direction(&self) -> Option<DirectionAdvanced> {
        match self.frame_grid.outer_key() {
            AnimationState::Idle => Some(self.frame_grid.inner_key()),
            AnimationState::Walk => None,
        }
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
            let direction = DirectionAdvanced::from(point);
            self.frame_grid.update_inner(direction);
        }
    }
}
