use bevy::prelude::*;
use bevy_yoleck::vpeol_2d::yoleck_vpeol_position_edit_adapter;
use bevy_yoleck::{egui, YoleckEdit, YoleckExtForApp, YoleckPopulate, YoleckTypeHandler};
use serde::{Deserialize, Serialize};

use crate::loading::GameAssets;

pub struct FloatingTextPlugin;

impl Plugin for FloatingTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_handler({
            YoleckTypeHandler::<FloatingText>::new("FloatingText")
                .with(yoleck_vpeol_position_edit_adapter(
                    |text: &mut FloatingText| {
                        bevy_yoleck::vpeol_2d::YoleckVpeolTransform2dProjection {
                            translation: &mut text.position,
                        }
                    },
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

fn populate(mut populate: YoleckPopulate<FloatingText>, game_assets: Res<GameAssets>) {
    populate.populate(|ctx, data, mut cmd| {
        let text = if ctx.is_in_editor() && data.text.trim_start().is_empty() {
            "<<TEXT>>".to_owned()
        } else {
            data.text.clone()
        };
        cmd.insert_bundle(Text2dBundle {
            text: {
                Text::with_section(
                    text,
                    TextStyle {
                        font: game_assets.font.clone(),
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
