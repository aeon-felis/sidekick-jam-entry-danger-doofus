use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::{YoleckExtForApp, YoleckPopulate, YoleckTypeHandlerFor};
use serde::{Deserialize, Serialize};

use crate::global_types::{AppState, ColorCode, CrystalState, GateState, IsPlatform};
use crate::yoleck_utils::{color_code_adapter, position_adapter, GRANULARITY};

pub struct GatePlugin;

impl Plugin for GatePlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_handler({
            YoleckTypeHandlerFor::<Gate>::new("Gate")
                .populate_with(populate)
                .with(position_adapter(
                    |gate: &mut Gate| (&mut gate.position, 1, 1),
                    -1.0,
                ))
                .with(color_code_adapter(|gate: &mut Gate| &mut gate.color_code))
        });
        app.add_system(update_gates_status);
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(move_gates));
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Gate {
    #[serde(default)]
    position: Vec2,
    #[serde(default)]
    color_code: ColorCode,
}

fn populate(mut populate: YoleckPopulate<Gate>, asset_server: Res<AssetServer>) {
    populate.populate(|_ctx, data, mut cmd| {
        let transform = Transform::from_xyz(
            data.position.x + 0.5 * GRANULARITY,
            data.position.y + 0.5 * GRANULARITY,
            -1.0,
        );
        cmd.insert(GateState {
            y_when_closed: transform.translation.y,
            is_open: false,
        });
        cmd.insert(IsPlatform);
        cmd.insert_bundle(SpriteBundle {
            transform,
            sprite: Sprite {
                custom_size: Some(Vec2::new(GRANULARITY, GRANULARITY)),
                color: data.color_code.bevy_color(),
                ..Default::default()
            },
            texture: asset_server.load("sprites/gate.png"),
            ..Default::default()
        });
        cmd.insert(RigidBody::KinematicPositionBased);
        cmd.insert(Collider::cuboid(0.5 * GRANULARITY, 0.5 * GRANULARITY));
    });
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
