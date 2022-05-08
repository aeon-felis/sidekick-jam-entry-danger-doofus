use bevy::prelude::*;

use crate::global_types::Facing;

pub struct AnimationHelpersPlugin;

impl Plugin for AnimationHelpersPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(match_animation_to_facing);
    }
}

fn match_animation_to_facing(mut query: Query<(&Facing, &mut Sprite)>) {
    for (facing, mut sprite) in query.iter_mut() {
        sprite.flip_x = *facing == Facing::Left;
    }
}
