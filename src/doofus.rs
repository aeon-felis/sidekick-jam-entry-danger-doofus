use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_egui::egui;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::YoleckSource;
use serde::{Deserialize, Serialize};

use crate::global_types::{
    AppState, Facing, IsCrystalActivator, IsDoofus, IsPlatform, IsSpringBoard,
};
use crate::utils::{entities_ordered_by_type, some_or};
use crate::yoleck_utils::{facing_edit, position_edit, position_to_transform, GRANULARITY};

pub struct DoofusPlugin;

impl Plugin for DoofusPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(propel_doofus));
    }
}

#[derive(Serialize, Deserialize)]
pub struct Doofus {
    #[serde(default)]
    position: Vec2,
    #[serde(default)]
    facing: Facing,
}

impl YoleckSource for Doofus {
    fn populate(&self, ctx: &bevy_yoleck::YoleckPopulateContext, cmd: &mut EntityCommands) {
        cmd.insert(IsDoofus);
        cmd.insert(IsCrystalActivator);
        cmd.insert(self.facing);
        cmd.insert_bundle(SpriteBundle {
            transform: position_to_transform(self.position.extend(0.0), 1, 1),
            sprite: Sprite {
                custom_size: Some(Vec2::new(GRANULARITY, GRANULARITY)),
                flip_x: self.facing == Facing::Left,
                ..Default::default()
            },
            texture: ctx.asset_server.load("sprites/doofus.png"),
            ..Default::default()
        });
        cmd.insert(RigidBody::Dynamic);
        cmd.insert(Collider::cuboid(0.5 * GRANULARITY, 0.5 * GRANULARITY));
        cmd.insert(GravityScale(0.2));
        cmd.insert(ActiveEvents::COLLISION_EVENTS);
        cmd.insert(Velocity::default());
        cmd.insert(LockedAxes::ROTATION_LOCKED);
    }

    fn edit(&mut self, ctx: &bevy_yoleck::YoleckEditContext, ui: &mut egui::Ui) {
        position_edit(ctx, ui, &mut self.position, 1, 1);
        facing_edit(ui, &mut self.facing);
    }
}

fn propel_doofus(
    mut doofus_query: Query<(&mut Velocity, &mut Facing), With<IsDoofus>>,
    platform_query: Query<(), With<IsPlatform>>,
    springboard_query: Query<(), With<IsSpringBoard>>,
    mut collision_events: EventReader<CollisionEvent>,
    rapier_context: Res<RapierContext>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                let [doofus_entity, other_entity] = some_or!(entities_ordered_by_type!(
                        [*entity1, *entity2],
                        doofus_query,
                ); continue);
                let contact_pair =
                    some_or!(rapier_context.contact_pair(doofus_entity, other_entity); continue);
                for manifold in contact_pair.manifolds() {
                    let normal = if doofus_entity == *entity1 {
                        -manifold.normal()
                    } else {
                        manifold.normal()
                    };
                    let doty = normal.dot(Vec2::Y);
                    if let Ok((mut velocity, mut facing)) = doofus_query.get_mut(doofus_entity) {
                        if 0.5 < doty {
                            if springboard_query.contains(other_entity) {
                                velocity.linvel = Vec2::new(facing.signum() * 2.0, 9.0);
                            } else if platform_query.contains(other_entity) {
                                velocity.linvel = Vec2::new(facing.signum() * 2.0, 3.0);
                            }
                        } else if doty.abs() <= 0.5 && normal.x.signum() != facing.signum() {
                            #[allow(clippy::collapsible_if)]
                            if platform_query.contains(other_entity) {
                                *facing = facing.reverse();
                                velocity.linvel.x = -velocity.linvel.x;
                            }
                        }
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}
