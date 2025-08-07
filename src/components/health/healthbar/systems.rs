use bevy::prelude::*;

use super::super::*;
use super::*;

pub fn update_healthbar(
    healthbar_query: Query<(&mut Transform, &mut Sprite, &ChildOf), With<marker::Healthbar>>,
    carrier_query: Query<(&component::Health, &Sprite, &Transform)>,
) {
    for (mut _healthbar_transform, healthbar_sprite, child_of) in healthbar_query {
        if let Ok((health, mut _sprite, _transform)) = carrier_query.get(child_of.parent()) {
            // TODO: Respect carrier's Sprite-s .custom_size(of there's .image) -> .image.size
            // TODO: Respect carrier's Transform (scale, rotation (when rect is being rotated it may overlap healthbar))

            healthbar_sprite.custom_size.unwrap().x =
                marker::Healthbar::MAX_WIDTH * health.percent_ratio();
        }
    }
}
