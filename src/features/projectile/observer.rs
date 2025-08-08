use avian2d::prelude::*;
use bevy::prelude::*;

use super::*;

use crate::{
    components::health::Health,
    features::creature::markers::{Creature, creature_type::Player},
};

pub fn observe_collision_with_creature(
    trigger: Trigger<OnCollisionStart>,
    creature_health_q: Query<&mut Health, (With<Creature>, Without<Player>)>,
    projectile_q: Query<&component::Projectile>,
    mut commands: Commands,
) {
    let projectile = trigger.target();
    let creature = trigger.collider;

    if creature_health_q.contains(creature) {
        let mut health = creature_health_q.get_inner(creature).unwrap();
        let &component::Projectile { damage } = projectile_q.get_inner(projectile).unwrap();

        match health.take_damage(damage) {
            crate::components::health::TakeDamageResult::Dead => {
                commands.entity(creature).despawn();
            }
            _ => {
                // Alive!
            }
        };
        commands.entity(projectile).despawn();
    }
}
