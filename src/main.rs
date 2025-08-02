use bevy::prelude::*;

mod app;

fn main() -> AppExit {
    App::new().add_plugins(app::AppPlugin).run()
}
