use bevy::prelude::*;

use super::*;

pub fn tick_animation_timer(time: Res<Time>, mut query: Query<&mut Animation>) {
    for mut animation in &mut query {
        animation.update_timer(time.delta());
    }
}

pub fn update_sprite_animation(mut query: Query<(&Animation, &mut Sprite)>) {
    for (animation, mut sprite) in &mut query {
        // Is switched to next frame. Syncing with atlas frame
        if animation.changed() {
            let atlas = sprite.texture_atlas.as_mut().unwrap();

            atlas.index = usize::from(animation.frame());
        }
    }
}
