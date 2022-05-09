use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::{egui, YoleckSource};
use serde::{Deserialize, Serialize};

use crate::global_types::IsIna;
use crate::player_control::PlayerControl;
use crate::yoleck_utils::{position_edit, position_to_transform, GRANULARITY};

pub struct InaPlugin;

impl Plugin for InaPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Serialize, Deserialize)]
pub struct Ina {
    #[serde(default)]
    position: Vec2,
}

impl YoleckSource for Ina {
    fn populate(&self, ctx: &bevy_yoleck::YoleckPopulateContext, cmd: &mut EntityCommands) {
        cmd.insert(IsIna);
        cmd.insert_bundle(SpriteBundle {
            transform: position_to_transform(self.position, 1, 1),
            sprite: Sprite {
                custom_size: Some(Vec2::new(GRANULARITY, GRANULARITY)),
                ..Default::default()
            },
            texture: ctx.asset_server.load("sprites/ina.png"),
            ..Default::default()
        });
        cmd.insert(RigidBody::Dynamic);
        cmd.insert(Collider::cuboid(0.25 * GRANULARITY, 0.5 * GRANULARITY));
        cmd.insert(GravityScale(0.2));
        cmd.insert(ActiveEvents::COLLISION_EVENTS);
        cmd.insert(Velocity::default());
        cmd.insert(LockedAxes::ROTATION_LOCKED);
        cmd.insert(PlayerControl::default());
    }

    fn edit(&mut self, ctx: &bevy_yoleck::YoleckEditContext, ui: &mut egui::Ui) {
        position_edit(ctx, ui, &mut self.position, 1, 1);
    }
}
