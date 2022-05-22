use bevy::prelude::*;
use bevy_yoleck::tools_2d::handle_position_fixed_z;
use bevy_yoleck::{egui, YoleckEdit, YoleckExtForApp, YoleckPopulate, YoleckTypeHandlerFor};
use serde::{Deserialize, Serialize};

use crate::yoleck_utils::GRANULARITY;

pub struct FloatingTextPlugin;

impl Plugin for FloatingTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_handler({
            YoleckTypeHandlerFor::<FloatingText>::new("FloatingText")
                .with(handle_position_fixed_z(
                    |text: &mut FloatingText| &mut text.position,
                    10.0,
                ))
                .populate_with(populate)
                .edit_with(edit)
        });
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct FloatingText {
    #[serde(default)]
    position: Vec2,
    #[serde(default)]
    text: String,
    #[serde(default = "default_scale")]
    scale: f32,
}

fn default_scale() -> f32 {
    0.05
}

fn populate(mut populate: YoleckPopulate<FloatingText>, asset_server: Res<AssetServer>) {
    populate.populate(|ctx, data, mut cmd| {
        if ctx.is_in_editor() {
            cmd.insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.0, 0.0, 0.0, 0.5),
                    custom_size: Some(Vec2::ONE * GRANULARITY / data.scale),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
        cmd.insert_bundle(Text2dBundle {
            text: {
                Text::with_section(
                    data.text.clone(),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 72.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        ..Default::default()
                    },
                )
            },
            transform: Transform {
                translation: data.position.extend(10.0),
                rotation: Default::default(),
                scale: Vec3::new(data.scale, data.scale, 1.0),
            },
            ..Default::default()
        });
    });
}

fn edit(mut edit: YoleckEdit<FloatingText>) {
    edit.edit(|_ctx, data, ui| {
        ui.text_edit_multiline(&mut data.text);
        ui.add(egui::Slider::new(&mut data.scale, 0.005..=0.05).logarithmic(true));
    });
}
