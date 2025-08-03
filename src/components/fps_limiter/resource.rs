use bevy::prelude::*;
use bevy_framepace::{FramepaceSettings, Limiter};

use crate::shared::data_structures::RepeatedState;

use super::config::FpsLimiterConfig;

#[derive(Resource)]
pub struct FpsLimiter {
    limits: RepeatedState<Limiter>,
}
impl FromWorld for FpsLimiter {
    fn from_world(world: &mut World) -> Self {
        let settings = world.get_resource_mut::<FramepaceSettings>();
        let limiter = Self::new(FpsLimiterConfig::default());

        match settings {
            Some(mut settings) => {
                settings.limiter = limiter.current();
            }
            None => {
                panic!(
                    "FramepaceSettings resource must be here already! Consider check resource load ordering"
                );
            }
        }

        limiter
    }
}

impl FpsLimiter {
    pub fn new(config: FpsLimiterConfig) -> Self {
        Self {
            limits: Self::get_limits(config),
        }
    }

    #[inline]
    pub fn current(&self) -> Limiter {
        self.limits.state()
    }

    pub fn to_next(&mut self) -> Limiter {
        self.limits.next()
    }

    fn get_limits(config: FpsLimiterConfig) -> RepeatedState<Limiter> {
        let mut limits = Self::get_special_limits(&config);

        limits.extend(Self::get_numbered_limits(&config));

        RepeatedState::new(limits)
    }

    fn get_numbered_limits(config: &FpsLimiterConfig) -> Vec<Limiter> {
        match config {
            FpsLimiterConfig::OnOff(fps_limits) | FpsLimiterConfig::AlwaysOn(fps_limits) => {
                fps_limits
                    .into_iter()
                    .map(|fps| Limiter::from_framerate(fps.clone()))
                    .collect()
            }
            FpsLimiterConfig::ConstantOn(fps) => vec![Limiter::from_framerate(fps.clone())],
            _ => vec![],
        }
    }
    fn get_special_limits(config: &FpsLimiterConfig) -> Vec<Limiter> {
        match config {
            FpsLimiterConfig::OnOff(_) | FpsLimiterConfig::Off => vec![Limiter::Off],
            _ => vec![],
        }
    }
}
