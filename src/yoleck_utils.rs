use bevy::prelude::*;
use bevy_egui::egui;
use bevy_yoleck::{YoleckEdit, YoleckPopulate, YoleckTypeHandlerFor};

use crate::global_types::ColorCode;

pub const GRANULARITY: f32 = 1.0;

pub fn round_to_tick(number: f32, tick: f32) -> f32 {
    (number / tick).round() * tick
}

pub fn round_vec2_to_tick(vec: Vec2, tick: f32) -> Vec2 {
    Vec2::new(round_to_tick(vec.x, tick), round_to_tick(vec.y, tick))
}

pub fn position_adapter<T: 'static>(
    projection: impl 'static + Clone + Send + Sync + Fn(&mut T) -> (&mut Vec2, usize, usize),
    z: f32,
) -> impl FnOnce(YoleckTypeHandlerFor<T>) -> YoleckTypeHandlerFor<T> {
    move |handler| {
        handler
            .populate_with({
                let projection = projection.clone();
                move |mut populate: YoleckPopulate<T>| {
                    populate.populate(|_ctx, data, mut cmd| {
                        let (position, width, height) = projection(data);
                        cmd.insert(Transform::from_xyz(
                            position.x + 0.5 * GRANULARITY * width as f32,
                            position.y + 0.5 * GRANULARITY * height as f32,
                            z,
                        ));
                    });
                }
            })
            .edit_with(move |mut edit: YoleckEdit<T>| {
                edit.edit(|ctx, data, ui| {
                    let (position, width, height) = projection(data);
                    if let Some(pos) = ctx.get_passed_data::<Vec2>() {
                        let pos = *pos;
                        let pos = pos - 0.5 * GRANULARITY * Vec2::new(width as f32, height as f32);
                        *position = pos;
                    }
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut position.x)
                                .prefix("X:")
                                .speed(0.05),
                        );
                        ui.add(
                            egui::DragValue::new(&mut position.y)
                                .prefix("Y:")
                                .speed(0.05),
                        );
                    });
                    *position = round_vec2_to_tick(*position, GRANULARITY);
                });
            })
    }
}

pub fn color_code_adapter<T: 'static>(
    projection: impl 'static + Clone + Send + Sync + Fn(&mut T) -> &mut ColorCode,
) -> impl FnOnce(YoleckTypeHandlerFor<T>) -> YoleckTypeHandlerFor<T> {
    move |handler| {
        handler
            .populate_with({
                let projection = projection.clone();
                move |mut populate: YoleckPopulate<T>| {
                    populate.populate(|_ctx, data, mut cmd| {
                        let color_code = projection(data);
                        cmd.insert(*color_code);
                    })
                }
            })
            .edit_with(move |mut edit: YoleckEdit<T>| {
                edit.edit(|_ctx, data, ui| {
                    let color_code = projection(data);
                    egui::ComboBox::from_label("")
                        .selected_text(color_code.egui_rich_text())
                        .show_ui(ui, |ui| {
                            for item in ColorCode::items() {
                                ui.selectable_value(color_code, item, item.egui_rich_text());
                            }
                        });
                })
            })
    }
}
