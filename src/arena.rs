use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::{egui, YoleckSource};
use serde::{Deserialize, Serialize};

struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Serialize, Deserialize)]
pub struct Floor {
    #[serde(default)]
    position: Vec2,
}

impl YoleckSource for Floor {
    fn populate(&self, ctx: &bevy_yoleck::YoleckPopulateContext, cmd: &mut EntityCommands) {
        cmd.insert_bundle(SpriteBundle {
            transform: Transform::from_translation(self.position.extend(0.0)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..Default::default()
            },
            texture: ctx.asset_server.load("sprites/floor-block.png"),
            ..Default::default()
        });
        cmd.insert(RigidBody::Fixed);
        cmd.insert(Collider::cuboid(0.5, 0.5));
    }

    fn edit(&mut self, ctx: &bevy_yoleck::YoleckEditContext, _ui: &mut egui::Ui) {
        if let Some(pos) = ctx.get_passed_data::<Vec2>() {
            *self.position = **pos;
        }
    }
}
