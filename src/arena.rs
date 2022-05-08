use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::{egui, YoleckSource};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

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
        let tile_size = Vec2::new(1.0, 1.0);
        let size = Vec2::new(
            self.width as f32 * tile_size.x,
            self.height as f32 * tile_size.y,
        );
        let center = self.position + 0.5 * size;
        cmd.insert_bundle(SpriteBundle {
            transform: Transform::from_translation(center.extend(0.0)),
            sprite: Sprite {
                color: Color::NONE,
                custom_size: Some(size),
                ..Default::default()
            },
            ..Default::default()
        });
        cmd.insert(RigidBody::Fixed);
        cmd.insert(Collider::cuboid(0.5, 0.5));

        if ctx.is_first_time() || self.prev_dimenstions != [self.width, self.height] {
            cmd.despawn_descendants();
            cmd.with_children(|commands| {
                let first_tile_center = 0.5 * (-size + tile_size);
                for (w, h) in (0..self.width).cartesian_product(0..self.height) {
                    commands.spawn_bundle(SpriteBundle {
                        transform: {
                            Transform::from_xyz(
                                first_tile_center.x + w as f32 * tile_size.x,
                                first_tile_center.y + h as f32 * tile_size.y,
                                0.0,
                            )
                        },
                        sprite: Sprite {
                            custom_size: Some(tile_size),
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
        if let Some(pos) = ctx.get_passed_data::<Vec2>() {
            let pos = *pos;
            let pos = pos - 0.5 * Vec2::new(self.width as f32, self.height as f32);
            *self.position = *round_vec2_to_tick(pos, 1.0);
        }
        self.prev_dimenstions = [self.width, self.height];
        ui.horizontal(|ui| {
            for (caption, value) in [("Width:", &mut self.width), ("Height:", &mut self.height)] {
                ui.add(egui::DragValue::new(value).prefix(caption).speed(0.05));
            }
        });
    }
}

pub fn round_to_tick(number: f32, tick: f32) -> f32 {
    (number / tick).round() * tick
}

pub fn round_vec2_to_tick(vec: Vec2, tick: f32) -> Vec2 {
    Vec2::new(round_to_tick(vec.x, tick), round_to_tick(vec.y, tick))
}
