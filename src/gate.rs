use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::YoleckSource;
use serde::{Deserialize, Serialize};

use crate::global_types::{ColorCode, IsGate, IsPlatform};
use crate::yoleck_utils::{color_code_edit, position_edit, position_to_transform, GRANULARITY};

#[derive(Serialize, Deserialize)]
pub struct Gate {
    #[serde(default)]
    position: Vec2,
    #[serde(default)]
    color_code: ColorCode,
}

impl YoleckSource for Gate {
    fn populate(
        &self,
        ctx: &bevy_yoleck::YoleckPopulateContext,
        cmd: &mut bevy::ecs::system::EntityCommands,
    ) {
        cmd.insert(IsGate);
        cmd.insert(IsPlatform);
        cmd.insert_bundle(SpriteBundle {
            transform: position_to_transform(self.position, 1, 1),
            sprite: Sprite {
                custom_size: Some(Vec2::new(GRANULARITY, GRANULARITY)),
                color: self.color_code.bevy_color(),
                ..Default::default()
            },
            texture: ctx.asset_server.load("sprites/gate.png"),
            ..Default::default()
        });
        cmd.insert(RigidBody::KinematicPositionBased);
        cmd.insert(Collider::cuboid(0.5 * GRANULARITY, 0.5 * GRANULARITY));
    }

    fn edit(&mut self, ctx: &bevy_yoleck::YoleckEditContext, ui: &mut bevy_egui_kbgp::egui::Ui) {
        position_edit(ctx, ui, &mut self.position, 1, 1);
        color_code_edit(ui, &mut self.color_code);
    }
}
