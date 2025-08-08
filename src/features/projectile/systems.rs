use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::RigidBodyVelocity};

use crate::{
    components::_animovement::{Animation, Movement},
    shared::data_structures::RangeDoubleMapper,
};

use super::*;

pub fn spawn_event_projectiles(
    mut commands: Commands,
    mut events: EventReader<events::SendProjectile>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for pending_projectile in events.read() {
        let events::SendProjectile {
            from,
            speed,
            intent,
        } = pending_projectile;

        let layout = TextureAtlasLayout::from_grid(UVec2::splat(128), 10, 6, None, None);

        commands
            .spawn((
                Name::new("Projectile"),
                marker::Projectile,
                Movement {
                    // Maximizing the intent for maximum speed mult
                    intent: Vec2::from_angle(intent.to_angle()),
                    max_speed: *speed,
                },
                Animation::new_with_animation_interval(
                    RangeDoubleMapper::new(|_| |_| 0..6),
                    Duration::from_millis(30),
                ),
                Sprite {
                    image: asset_server
                        .load("sprites/effects/magic/magic-bolts/fireball/tile_map2.png"),
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas_layouts.add(layout),
                        index: 0,
                    }),
                    ..default()
                },
                Transform {
                    translation: from.translation,
                    rotation: Quat::from_rotation_z(intent.to_angle()),
                    scale: Vec3::splat(2.0),
                },
            ))
            // Physics
            .insert((
                // Doesn't work even if you remove Movement component. TODO: make it work
                Velocity::linear(Vec2::from_angle(intent.to_angle()) * *speed),
                RigidBody::Fixed,
                Sensor,
                Collider::cuboid(10.0, 10.0),
                Restitution::coefficient(1.0),
            ));
    }
}

pub mod debug {
    use super::*;

    pub fn remove_all_projectiles_by_event(
        mut commands: Commands,
        mut events: EventReader<events::debug::RemoveAllProjectiles>,
        projectiles: Query<Entity, With<marker::Projectile>>,
    ) {
        for _ in events.read() {
            for projectile_entity in projectiles {
                commands.entity(projectile_entity).despawn();
            }
        }
    }
}
