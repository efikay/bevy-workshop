use bevy::prelude::*;

use super::super::*;
use super::*;

pub fn update_healthbar(
    healthbar_query: Query<(&mut Transform, &mut Sprite, &ChildOf), With<marker::Healthbar>>,
    carrier_query: Query<(&component::Health, &Sprite, &Transform), Without<marker::Healthbar>>,
) {
    for (mut healthbar_transform, healthbar_sprite, child_of) in healthbar_query {
        if let Ok((health, mut _sprite, transform)) = carrier_query.get(child_of.parent()) {
            // TODO: Respect carrier's Sprite-s .custom_size(of there's .image) -> .image.size
            // TODO: Respect carrier's Transform (scale, rotation (when rect is being rotated it may overlap healthbar))

            // Setting healthbar width depending on how much HP left
            healthbar_sprite.custom_size.unwrap().x =
                marker::Healthbar::MAX_WIDTH * health.percent_ratio();
            
            // For constant healthbar size (non-depending on parent scale)
            // TODO: Some better solutions like GlobalZIndex but for transform's scale?
            healthbar_transform.scale = transform.scale.recip();
        }
    }
}
