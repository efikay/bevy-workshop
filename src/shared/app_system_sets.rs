use bevy::ecs::schedule::SystemSet;

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}
impl AppSystems {
    pub const fn system_set() -> (AppSystems, AppSystems, AppSystems) {
        (
            AppSystems::TickTimers,
            AppSystems::RecordInput,
            AppSystems::Update,
        )
    }
}

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct AppPausableSystems;
