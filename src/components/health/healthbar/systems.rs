use bevy::prelude::*;

use super::super::*;
use super::*;

pub fn update_healthbar(
    healthbar_query: Query<(&mut Transform, &mut Sprite, &ChildOf), With<marker::Healthbar>>,
    carrier_query: Query<(&component::Health, &Sprite, &Transform), Without<marker::Healthbar>>,
) {
    for (mut healthbar_transform, mut healthbar_sprite, child_of) in healthbar_query {
        if let Ok((health, _sprite, transform)) = carrier_query.get(child_of.parent()) {
            // TODO: Respect carrier's Sprite-s .custom_size(if there's .image) -> .image.size (omg image is the whole texture atlas image)
            // TODO: Respect carrier's Transform (scale, rotation (when rect is being rotated it may overlap healthbar))

            // Setting healthbar width depending on how much HP left
            healthbar_sprite.custom_size = Some(Vec2::new(
                marker::Healthbar::MAX_WIDTH * health.percent_ratio(),
                marker::Healthbar::HEIGHT,
            ));

            // For constant healthbar size (non-depending on parent scale)
            // TODO: Some better solutions like GlobalZIndex but for transform's scale?
            healthbar_transform.scale = transform.scale.recip();

            // TODO: Calculate trans_y correctly (image size is tricky in sprites with atlases)
            healthbar_transform.translation.y = marker::Healthbar::PADDING_Y;
        }
    }
}

pub mod debug {
    use super::*;

    const DAMAGE_PER_TICK: u32 = 20;

    pub fn take_damage_if_alive(health_components: Query<&mut component::Health>) {
        for mut health in health_components {
            if !health.is_dead() {
                health.take_damage(DAMAGE_PER_TICK);
            }
        }
    }
}
