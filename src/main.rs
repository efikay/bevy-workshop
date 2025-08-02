use bevy::prelude::*;

mod app;
mod shared;

fn main() -> AppExit {
    App::new().add_plugins(app::AppPlugin).run()
}
