use bevy::prelude::*;
use bevy_egui::egui;

use crate::global_types::Facing;

pub const GRANULARITY: f32 = 1.0;

pub fn round_to_tick(number: f32, tick: f32) -> f32 {
    (number / tick).round() * tick
}

pub fn round_vec2_to_tick(vec: Vec2, tick: f32) -> Vec2 {
    Vec2::new(round_to_tick(vec.x, tick), round_to_tick(vec.y, tick))
}

pub fn position_edit(
    ctx: &bevy_yoleck::YoleckEditContext,
    ui: &mut egui::Ui,
    position: &mut Vec2,
    width: usize,
    height: usize,
) {
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
}

pub fn position_to_transform(position: Vec2, width: usize, height: usize) -> Transform {
    Transform::from_xyz(
        position.x + 0.5 * GRANULARITY * width as f32,
        position.y + 0.5 * GRANULARITY * height as f32,
        0.0,
    )
}

pub fn facing_edit(ui: &mut egui::Ui, facing: &mut Facing) {
    ui.horizontal(|ui| {
        ui.label("Facing:");
        ui.selectable_value(facing, Facing::Left, "<-");
        ui.selectable_value(facing, Facing::Right, "->");
    });
}
