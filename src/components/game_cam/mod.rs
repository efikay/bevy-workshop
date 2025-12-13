use bevy::prelude::*;

mod bundle;
mod systems;
mod marker;

pub struct GameCamPlugin;
impl Plugin for GameCamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(bundle::bundle());
        });
    }
}
