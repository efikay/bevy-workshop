use bevy::{ecs::query::QuerySingleError, prelude::*};

use super::marker::CameraTarget;

pub fn camera_follow_target(
    mut camera_transform: Single<&mut Transform, (With<Camera>, Without<CameraTarget>)>,
    targets: Query<&Transform, (With<CameraTarget>, Without<Camera>)>,
) {
    for target_transform in targets {
        camera_transform.translation = target_transform.translation.clone();

        // Only first active camera target gets what it wants
        // TODO: Later will try to target multiple components at once (check: if fits the screen and stuff)
        // TODO: Smooth follow (change translation in smooth way each frame)
        break;
    }
}
