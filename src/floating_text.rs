use bevy::prelude::*;
use bevy_yoleck::{egui, YoleckSource};
use serde::{Deserialize, Serialize};

use crate::yoleck_utils::GRANULARITY;

#[derive(Serialize, Deserialize)]
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

impl YoleckSource for FloatingText {
    fn populate(
        &self,
        ctx: &bevy_yoleck::YoleckPopulateContext,
        cmd: &mut bevy::ecs::system::EntityCommands,
    ) {
        if ctx.is_in_editor() {
            cmd.insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.0, 0.0, 0.0, 0.5),
                    custom_size: Some(Vec2::ONE * GRANULARITY / self.scale),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
        cmd.insert_bundle(Text2dBundle {
            text: {
                Text::with_section(
                    self.text.clone(),
                    TextStyle {
                        font: ctx.asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 72.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        ..Default::default()
                    },
                )
            },
            transform: Transform {
                translation: self.position.extend(10.0),
                rotation: Default::default(),
                scale: Vec3::new(self.scale, self.scale, 1.0),
            },
            ..Default::default()
        });
    }

    fn edit(&mut self, ctx: &bevy_yoleck::YoleckEditContext, ui: &mut egui::Ui) {
        if let Some(pos) = ctx.get_passed_data::<Vec2>() {
            self.position = *pos;
        }
        ui.horizontal(|ui| {
            ui.add(
                egui::DragValue::new(&mut self.position.x)
                    .prefix("X:")
                    .speed(0.05),
            );
            ui.add(
                egui::DragValue::new(&mut self.position.y)
                    .prefix("Y:")
                    .speed(0.05),
            );
        });
        ui.text_edit_multiline(&mut self.text);
        ui.add(egui::Slider::new(&mut self.scale, 0.005..=0.05).logarithmic(true));
    }
}
