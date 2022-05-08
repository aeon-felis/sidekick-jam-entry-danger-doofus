use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::{egui, YoleckSource};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::yoleck_utils::{position_edit, position_to_transform, GRANULARITY};

struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    #[serde(default)]
    position: Vec2,
    #[serde(default = "the_fucking_number_one_why_cant_serde_accept_literals")]
    width: usize,
    #[serde(default = "the_fucking_number_one_why_cant_serde_accept_literals")]
    height: usize,
    #[serde(default, skip)]
    prev_dimenstions: [usize; 2],
}

fn the_fucking_number_one_why_cant_serde_accept_literals() -> usize {
    1
}

impl YoleckSource for Block {
    fn populate(&self, ctx: &bevy_yoleck::YoleckPopulateContext, cmd: &mut EntityCommands) {
        let size = Vec2::new(
            self.width as f32 * GRANULARITY,
            self.height as f32 * GRANULARITY,
        );
        cmd.insert_bundle(SpriteBundle {
            transform: position_to_transform(self.position, self.width, self.height),
            sprite: Sprite {
                color: Color::NONE,
                custom_size: Some(size),
                ..Default::default()
            },
            ..Default::default()
        });
        cmd.insert(RigidBody::Fixed);
        cmd.insert(Collider::cuboid(0.5 * size.x, 0.5 * size.y));

        if ctx.is_first_time() || self.prev_dimenstions != [self.width, self.height] {
            cmd.despawn_descendants();
            cmd.with_children(|commands| {
                let first_tile_center = 0.5 * (-size + Vec2::ONE * GRANULARITY);
                for (w, h) in (0..self.width).cartesian_product(0..self.height) {
                    commands.spawn_bundle(SpriteBundle {
                        transform: {
                            Transform::from_xyz(
                                first_tile_center.x + w as f32 * GRANULARITY,
                                first_tile_center.y + h as f32 * GRANULARITY,
                                0.0,
                            )
                        },
                        sprite: Sprite {
                            custom_size: Some(Vec2::ONE * GRANULARITY),
                            ..Default::default()
                        },
                        texture: ctx.asset_server.load("sprites/block-tile.png"),
                        ..Default::default()
                    });
                }
            });
        }
    }

    fn edit(&mut self, ctx: &bevy_yoleck::YoleckEditContext, ui: &mut egui::Ui) {
        position_edit(ctx, ui, &mut self.position, self.width, self.height);
        self.prev_dimenstions = [self.width, self.height];
        ui.horizontal(|ui| {
            for (caption, value) in [("Width:", &mut self.width), ("Height:", &mut self.height)] {
                ui.add(egui::DragValue::new(value).prefix(caption).speed(0.05));
            }
        });
    }
}
