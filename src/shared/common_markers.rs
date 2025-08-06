use bevy::prelude::*;

/**
 * Common markers with explanations
 */
fn _doc() {}


/**
 * Everything marked with this will be moving from WASD during gameplay
 * in normal circumstances
 */
#[derive(Component, Reflect)]
#[require(Transform)]
pub struct WASD;
