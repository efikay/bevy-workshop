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
            is_ghost,
        } = pending_projectile;

        let layout = TextureAtlasLayout::from_grid(UVec2::splat(128), 10, 6, None, None);

        commands
            .spawn((
                Name::new("Projectile"),
                marker::Projectile,
                // Movement {
                // Maximizing the intent for maximum speed mult
                // intent: Vec2::from_angle(intent.to_angle()),
                // max_speed: *speed,
                // },
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
                if *is_ghost {
                    RigidBody::Fixed
                } else {
                    RigidBody::Dynamic
                },
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

    pub fn toggle_projectiles_body_type(
        projectiles: Query<&mut RigidBody, With<marker::Projectile>>,
        mut is_fixed: Local<bool>,
    ) {
        *is_fixed = !*is_fixed;

        for mut proj_body in projectiles {
            *proj_body = match *is_fixed {
                true => RigidBody::Fixed,
                false => RigidBody::Dynamic,
            }
        }
    }

    /// SCREAMING_SNAKE_CASE is intentional (it won't work that good otherwise)
    #[allow(non_snake_case)]
    pub fn CAST_FIRE_NOVA_by_event(
        mut reader: EventReader<events::debug::CastFireNova>,
        mut writer: EventWriter<events::SendProjectile>,
    ) {
        const FIREBALLS_AMOUNT: usize = 30;

        for fire_nova_request in reader.read() {
            let from = fire_nova_request.from;

            for radian in generate_directions(FIREBALLS_AMOUNT).into_iter() {
                writer.write(events::SendProjectile {
                    from,
                    intent: Vec2::from_angle(radian),
                    speed: 1000.0,
                    is_ghost: false,
                });
            }
        }
    }

    /// Generates N directions evenly spaced between -π and +π radians
    /// For N=4: [-π/4, π/4, 3π/4, -3π/4]
    /// For N=2: [0.0, π]
    fn generate_directions(n: usize) -> Vec<f32> {
        let mut directions = Vec::with_capacity(n);
        let angle_step = 2.0 * std::f32::consts::PI / n as f32;

        for i in 0..n {
            let angle = angle_step * i as f32;
            // Shift range from [0, 2π] to [-π, π]
            let shifted_angle = if angle > std::f32::consts::PI { angle - 2.0 * std::f32::consts::PI } else { angle };
            directions.push(shifted_angle);
        }

        directions
    }
}
