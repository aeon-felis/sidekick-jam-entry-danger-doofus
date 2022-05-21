use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use bevy_tweening::lens::{TransformPositionLens, TransformScaleLens};
use bevy_tweening::*;
use bevy_yoleck::{YoleckEdit, YoleckExtForApp, YoleckPopulate, YoleckTypeHandlerFor};
use serde::{Deserialize, Serialize};

use crate::global_types::{AppState, IsDoofus, IsDoor, TweenCompletedCode};
use crate::utils::{entities_ordered_by_type, some_or};
use crate::yoleck_utils::{position_edit, position_to_transform, GRANULARITY};

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_handler({
            YoleckTypeHandlerFor::<Door>::new("Door")
                .populate_with(populate)
                .edit_with(edit)
        });
        app.add_system_set({
            SystemSet::on_update(AppState::Game)
                .with_system(doofus_reach_door)
                .with_system(finish_level_when_go_through_door_animation_is_over)
        });
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Door {
    #[serde(default)]
    position: Vec2,
}

fn populate(mut populate: YoleckPopulate<Door>, asset_server: Res<AssetServer>) {
    populate.populate(|_ctx, data, mut cmd| {
        cmd.insert(IsDoor);
        cmd.insert_bundle(SpriteBundle {
            transform: position_to_transform(data.position.extend(-1.0), 1, 1),
            sprite: Sprite {
                custom_size: Some(Vec2::new(GRANULARITY, GRANULARITY)),
                ..Default::default()
            },
            texture: asset_server.load("sprites/door.png"),
            ..Default::default()
        });
        cmd.insert(RigidBody::Fixed);
        cmd.insert(Collider::cuboid(0.25 * GRANULARITY, 0.5 * GRANULARITY));
        cmd.insert(Sensor(true));
    });
}

fn edit(mut edit: YoleckEdit<Door>) {
    edit.edit(|ctx, data, ui| {
        position_edit(ctx, ui, &mut data.position, 1, 1);
    });
}

fn doofus_reach_door(
    mut commands: Commands,
    doofus_query: Query<&GlobalTransform, With<IsDoofus>>,
    mut collision_events: EventReader<CollisionEvent>,
    door_query: Query<&GlobalTransform, With<IsDoor>>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, flags) => {
                if !flags.intersects(CollisionEventFlags::SENSOR) {
                    continue;
                }
                let [doofus_entity, door_entity] = some_or!(entities_ordered_by_type!(
                        [*entity1, *entity2],
                        doofus_query,
                        door_query,
                ); continue);
                let mut cmd = commands.entity(doofus_entity);
                let doofus_transform = doofus_query.get(doofus_entity).unwrap();
                let door_transform = door_query.get(door_entity).unwrap();
                cmd.remove::<RigidBody>();
                cmd.remove::<Collider>();
                let animation_duration = Duration::from_secs(1);
                cmd.insert(Animator::new(Tracks::new([
                    Tween::new(
                        EaseMethod::EaseFunction(EaseFunction::QuarticIn),
                        TweeningType::Once,
                        animation_duration,
                        TransformScaleLens {
                            start: Vec3::ONE,
                            end: Vec3::ZERO,
                        },
                    )
                    .with_completed_event(true, TweenCompletedCode::ExitDoorFinished as u64),
                    Tween::new(
                        EaseMethod::Linear,
                        TweeningType::Once,
                        animation_duration,
                        TransformPositionLens {
                            start: doofus_transform.translation,
                            end: door_transform.translation - 0.25 * GRANULARITY * Vec3::Y,
                        },
                    ),
                ])));
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}

fn finish_level_when_go_through_door_animation_is_over(
    mut state: ResMut<State<AppState>>,
    mut event_reader: EventReader<TweenCompleted>,
) {
    for TweenCompleted {
        entity: _,
        user_data,
    } in event_reader.iter()
    {
        if *user_data != TweenCompletedCode::ExitDoorFinished as u64 {
            continue;
        }
        state.set(AppState::LevelCompleted).unwrap();
    }
}
