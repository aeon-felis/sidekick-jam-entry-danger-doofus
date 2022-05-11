use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::YoleckSource;
use serde::{Deserialize, Serialize};

use crate::global_types::{AppState, ColorCode, CrystalState, GateState, IsPlatform};
use crate::yoleck_utils::{color_code_edit, position_edit, position_to_transform, GRANULARITY};

pub struct GatePlugin;

impl Plugin for GatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_gates_status);
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(move_gates));
    }
}

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
        let transform = position_to_transform(self.position, 1, 1);
        cmd.insert(GateState {
            y_when_closed: transform.translation.y,
            is_open: false,
        });
        cmd.insert(self.color_code);
        cmd.insert(IsPlatform);
        cmd.insert_bundle(SpriteBundle {
            transform,
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

fn update_gates_status(
    crystal_query: Query<(&ColorCode, &CrystalState)>,
    mut gate_query: Query<(&ColorCode, &mut GateState)>,
) {
    let mut colors_activated = [false; ColorCode::size()];
    for (color_code, crystal_state) in crystal_query.iter() {
        if 0 < crystal_state.num_activators {
            colors_activated[*color_code as usize] = true;
        }
    }
    for (color_code, mut gate_state) in gate_query.iter_mut() {
        let should_be_open = colors_activated[*color_code as usize];
        gate_state.is_open = should_be_open;
    }
}

fn move_gates(time: Res<Time>, mut gate_query: Query<(&mut Transform, &GateState)>) {
    let speed = time.delta().as_secs_f32() * 2.0;
    for (mut transform, gate_state) in gate_query.iter_mut() {
        let current_y = transform.translation.y;
        let mut desired_y = gate_state.y_when_closed;
        if gate_state.is_open {
            desired_y -= GRANULARITY;
        }
        let y_diff = desired_y - current_y;
        if y_diff.abs() < 2.0 * speed {
            transform.translation.y = desired_y;
        } else {
            transform.translation.y += y_diff.signum() * speed;
        }
    }
}
