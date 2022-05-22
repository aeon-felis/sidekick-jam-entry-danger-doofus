use bevy::prelude::*;
use bevy_egui::egui;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::{YoleckEdit, YoleckExtForApp, YoleckPopulate, YoleckTypeHandlerFor};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::global_types::IsPlatform;
use crate::yoleck_utils::{position_adapter, GRANULARITY};

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_handler({
            YoleckTypeHandlerFor::<Block>::new("Block")
                .populate_with(populate)
                .with(position_adapter(
                    |block: &mut Block| (&mut block.position, block.width, block.height),
                    0.0,
                ))
                .edit_with(edit)
        });
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
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

fn populate(mut populate: YoleckPopulate<Block>, asset_server: Res<AssetServer>) {
    populate.populate(|ctx, data, mut cmd| {
        let size = Vec2::new(
            data.width as f32 * GRANULARITY,
            data.height as f32 * GRANULARITY,
        );
        cmd.insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::NONE,
                custom_size: Some(size),
                ..Default::default()
            },
            ..Default::default()
        });
        cmd.insert(RigidBody::Fixed);
        cmd.insert(Collider::cuboid(0.5 * size.x, 0.5 * size.y));
        cmd.insert(IsPlatform);

        if ctx.is_first_time() || data.prev_dimenstions != [data.width, data.height] {
            cmd.despawn_descendants();
            cmd.with_children(|commands| {
                let first_tile_center = 0.5 * (-size + Vec2::ONE * GRANULARITY);
                for (w, h) in (0..data.width).cartesian_product(0..data.height) {
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
                        texture: asset_server.load("sprites/block-tile.png"),
                        ..Default::default()
                    });
                }
            });
        }
    });
}

fn edit(mut edit: YoleckEdit<Block>) {
    edit.edit(|_ctx, data, ui| {
        data.prev_dimenstions = [data.width, data.height];
        ui.horizontal(|ui| {
            for (caption, value) in [("Width:", &mut data.width), ("Height:", &mut data.height)] {
                ui.add(egui::DragValue::new(value).prefix(caption).speed(0.05));
            }
        });
    });
}
