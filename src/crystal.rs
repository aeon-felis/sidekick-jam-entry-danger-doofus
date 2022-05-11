use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::YoleckSource;
use serde::{Deserialize, Serialize};

use crate::global_types::{ColorCode, CrystalState, IsCrystalActivator, IsPlatform};
use crate::utils::{entities_ordered_by_type, some_or};
use crate::yoleck_utils::{color_code_edit, position_edit, position_to_transform, GRANULARITY};

pub struct CrystalPlugin;

impl Plugin for CrystalPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_crystals_activation);
    }
}

#[derive(Serialize, Deserialize)]
pub struct Crystal {
    #[serde(default)]
    position: Vec2,
    #[serde(default)]
    color_code: ColorCode,
}

impl YoleckSource for Crystal {
    fn populate(
        &self,
        ctx: &bevy_yoleck::YoleckPopulateContext,
        cmd: &mut bevy::ecs::system::EntityCommands,
    ) {
        cmd.insert(CrystalState { num_activators: 0 });
        cmd.insert(self.color_code);
        cmd.insert(IsPlatform);
        cmd.insert_bundle(SpriteBundle {
            transform: position_to_transform(self.position, 1, 1),
            sprite: Sprite {
                custom_size: Some(Vec2::new(GRANULARITY, GRANULARITY)),
                color: self.color_code.bevy_color(),
                ..Default::default()
            },
            texture: ctx.asset_server.load("sprites/crystal-off.png"),
            ..Default::default()
        });
        cmd.insert(RigidBody::Fixed);
        cmd.insert(Collider::cuboid(0.25 * GRANULARITY, 0.5 * GRANULARITY));
        cmd.insert(Sensor(true));
    }

    fn edit(&mut self, ctx: &bevy_yoleck::YoleckEditContext, ui: &mut bevy_egui_kbgp::egui::Ui) {
        position_edit(ctx, ui, &mut self.position, 1, 1);
        color_code_edit(ui, &mut self.color_code);
    }
}

fn update_crystals_activation(
    mut collision_events: EventReader<CollisionEvent>,
    mut crystal_query: Query<(&mut CrystalState, &mut Handle<Image>)>,
    activator_query: Query<(), With<IsCrystalActivator>>,
    asset_server: Res<AssetServer>,
) {
    for collision_event in collision_events.iter() {
        let (entity1, entity2, intersection_started) = match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => (entity1, entity2, true),
            CollisionEvent::Stopped(entity1, entity2, _) => (entity1, entity2, false),
        };
        let [crystal_entity, _activator_entity] = some_or!(entities_ordered_by_type!(
                [*entity1, *entity2],
                crystal_query,
                activator_query,
        ); continue);
        let (mut crystal_state, mut crystal_texture) =
            crystal_query.get_mut(crystal_entity).unwrap();
        if intersection_started {
            crystal_state.num_activators += 1;
            if crystal_state.num_activators == 1 {
                *crystal_texture = asset_server.load("sprites/crystal-on.png");
            }
        } else {
            crystal_state.num_activators -= 1;
            if crystal_state.num_activators == 0 {
                *crystal_texture = asset_server.load("sprites/crystal-off.png");
            }
        }
    }
}
