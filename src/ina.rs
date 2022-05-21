use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::{YoleckEdit, YoleckExtForApp, YoleckPopulate, YoleckTypeHandlerFor};
use serde::{Deserialize, Serialize};

use crate::global_types::{IsCrystalActivator, IsIna, IsSpringBoard};
use crate::player_control::PlayerControl;
use crate::yoleck_utils::{position_edit, position_to_transform, GRANULARITY};

pub struct InaPlugin;

impl Plugin for InaPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_handler({
            YoleckTypeHandlerFor::<Ina>::new("Ina")
                .populate_with(populate)
                .edit_with(edit)
        });
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Ina {
    #[serde(default)]
    position: Vec2,
}

fn populate(mut populate: YoleckPopulate<Ina>, asset_server: Res<AssetServer>) {
    populate.populate(|_ctx, data, mut cmd| {
        cmd.insert(IsIna);
        cmd.insert(IsCrystalActivator);
        cmd.insert(IsSpringBoard);
        cmd.insert_bundle(SpriteBundle {
            transform: position_to_transform(data.position.extend(0.0), 1, 1),
            sprite: Sprite {
                custom_size: Some(Vec2::new(GRANULARITY, GRANULARITY)),
                ..Default::default()
            },
            texture: asset_server.load("sprites/ina.png"),
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

fn edit(mut edit: YoleckEdit<Ina>) {
    edit.edit(|ctx, data, ui| {
        position_edit(ctx, ui, &mut data.position, 1, 1);
    });
}
