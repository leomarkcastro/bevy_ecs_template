// To describe how the DissapearProximity component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use crate::{
    entity_factory::entities::{
        playerv2::entities::Playerv2Entity, playerv3::entities::Playerv3Entity,
    },
    utils::check_collide::check_pointtorect_collide_rect,
};

use super::DissapearProximityComponent;

pub struct DissapearProximityPlugin;

impl Plugin for DissapearProximityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(dissapear_proximity_system);
    }
}

// For now, it will only disappear when the player is within proximity
fn dissapear_proximity_system(
    mut q_roof: Query<(&mut Sprite, &GlobalTransform), With<DissapearProximityComponent>>,
    q_player: Query<
        (&TextureAtlasSprite, &GlobalTransform),
        (With<Playerv3Entity>, Without<DissapearProximityComponent>),
    >,
) {
    // println!("{}", q_player.iter().count().to_string());
    for (a_sprite, a_gtransform) in q_player.iter() {
        // check if any player is within detection range
        for (mut b_sprite, b_gtransform) in q_roof.iter_mut() {
            // get the vec3 of a and b from global transform
            let a_gt = a_gtransform.to_scale_rotation_translation().2;
            let b_gt = b_gtransform.to_scale_rotation_translation().2;

            let a_sprite_size = &a_sprite.custom_size.unwrap_or_default();
            let b_sprite_size = &b_sprite.custom_size.unwrap_or_default();
            let mut color = &mut b_sprite.color;

            // let distance = b_gt.truncate().distance(a_gt.truncate());

            // println!("{}", distance);
            if check_pointtorect_collide_rect(
                &a_gt.truncate(),
                &a_sprite_size,
                &b_gt.truncate(),
                &b_sprite_size,
            ) {
                // reduce the opacity of the sprite
                color.set_a(f32::max(color.a() - 0.01, 0.0));
            } else {
                // reset the opacity of the sprite
                color.set_a(f32::min(color.a() + 0.05, 1.0));
            }
        }
    }
}
