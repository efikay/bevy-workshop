use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

mod markers;
mod systems;

pub fn plugin(app: &mut App) {
    app.add_plugins(FrameTimeDiagnosticsPlugin::default());

    app.add_systems(Startup, systems::setup);
    app.add_systems(Update, systems::update_fps_value);
}
