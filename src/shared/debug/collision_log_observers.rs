use bevy::prelude::*;
use avian2d::prelude::*;

pub fn log_any_collision_start(trigger: Trigger<OnCollisionStart>) {
    let the_one_who_knocks = trigger.target();
    let victim = trigger.collider;

    log::info!("Trigger<OnCollisionStart>: Collision detected: {}->{}", the_one_who_knocks, victim);
}

pub fn log_any_collision_end(trigger: Trigger<OnCollisionEnd>) {
    let the_one_who_knocks = trigger.target();
    let victim = trigger.collider;

    log::info!("Trigger<OnCollisionEnd>: Collision ended: {}->{}", the_one_who_knocks, victim);
}
