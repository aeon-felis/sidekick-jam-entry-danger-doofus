use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::{YoleckExtForApp, YoleckPopulate, YoleckTypeHandlerFor};
use serde::{Deserialize, Serialize};

use crate::global_types::{IsCrystalActivator, IsIna, IsSpringBoard};
use crate::loading::GameAssets;
use crate::player_control::PlayerControl;
use crate::yoleck_utils::{position_adapter, GRANULARITY};

pub struct InaPlugin;

impl Plugin for InaPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_handler({
            YoleckTypeHandlerFor::<Ina>::new("Ina")
                .populate_with(populate)
                .with(position_adapter(
                    |ina: &mut Ina| (&mut ina.position, 1, 1),
                    0.0,
                ))
        });
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Ina {
    #[serde(default)]
    position: Vec2,
}

fn populate(mut populate: YoleckPopulate<Ina>, game_assets: Res<GameAssets>) {
    populate.populate(|_ctx, _data, mut cmd| {
        cmd.insert(IsIna);
        cmd.insert(IsCrystalActivator);
        cmd.insert(IsSpringBoard);
        cmd.insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(GRANULARITY, GRANULARITY)),
                ..Default::default()
            },
            texture: game_assets.ina.clone(),
            ..Default::default()
        });
        cmd.insert(RigidBody::Dynamic);
        cmd.insert(Collider::cuboid(0.25 * GRANULARITY, 0.5 * GRANULARITY));
        cmd.insert(GravityScale(0.2));
        cmd.insert(ActiveEvents::COLLISION_EVENTS);
        cmd.insert(Velocity::default());
        cmd.insert(LockedAxes::ROTATION_LOCKED);
        cmd.insert(PlayerControl::default());
    });
}
