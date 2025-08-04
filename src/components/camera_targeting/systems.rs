use bevy::prelude::*;

use super::marker::CameraTarget;

pub fn camera_follow_target(
    mut camera_transform: Single<&mut Transform, (With<Camera>, Without<CameraTarget>)>,
    targets: Query<(&Transform, &CameraTarget), (With<CameraTarget>, Without<Camera>)>,
) {
    for (target_transform, config) in targets {
        if config.is_enabled {
            camera_transform.translation = target_transform.translation.clone();

            // Only first active camera target gets what it wants
            break;
        }
    }
}
