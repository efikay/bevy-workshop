use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod events;
mod marker;
mod systems;

pub fn plugin(app: &mut App) {
    app.add_event::<events::SendProjectile>();
    app.register_type::<events::SendProjectile>();

    app.add_systems(Update, systems::spawn_event_projectiles);

    #[cfg(feature = "inspector__projectile")]
    app.add_plugins(debug::plugin);
}

mod debug {
    use bevy::input::common_conditions::input_just_pressed;
    use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;

    use super::*;

    pub fn plugin(app: &mut App) {
        app.add_plugins(FilterQueryInspectorPlugin::<With<marker::Projectile>>::default());

        app.add_event::<events::debug::RemoveAllProjectiles>();

        app.add_systems(Update, display_events);

        app.add_systems(Update, systems::debug::remove_all_projectiles_by_event);
        app.add_systems(
            Update,
            remove_all_projectiles.run_if(input_just_pressed(KeyCode::Enter)),
        );
    }

    /* A system that displays the events. */
    fn display_events(
        mut collision_events: EventReader<CollisionEvent>,
        mut contact_force_events: EventReader<ContactForceEvent>,
    ) {
        for collision_event in collision_events.read() {
            println!("Received collision event: {:?}", collision_event);
        }

        for contact_force_event in contact_force_events.read() {
            println!("Received contact force event: {:?}", contact_force_event);
        }
    }

    fn remove_all_projectiles(mut writer: EventWriter<events::debug::RemoveAllProjectiles>) {
        println!("Sending RemoveAllProjectiles event..");

        writer.write(events::debug::RemoveAllProjectiles);
    }
}
