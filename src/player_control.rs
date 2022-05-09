use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::global_types::AppState;

pub struct PlayerControlPlugin;

impl Plugin for PlayerControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(control_player));
        app.insert_resource(PlayerMovementSettings {
            max_speed: 10.0,
            impulse_exponent: 4.0,
            impulse_coefficient: 400.0,
            jump_power_coefficient: 10.0,
            jump_brake_coefficient: 0.02,
            start_fall_before_peak: 10.0,
            start_of_fall_range: 10.0,
            start_of_fall_gravity_boost: 30.0,
            fall_boost_coefficient: 1.06,
            stood_on_time_coefficient: 10.0,
            uphill_move_exponent: 0.5,
            downhill_brake_exponent: 1.0,
        });
    }
}

#[derive(Component)]
pub struct PlayerControl {
    mid_jump: bool,
    last_stood_on: Vec2,
    stood_on_potential: f32,
}

impl Default for PlayerControl {
    fn default() -> Self {
        Self {
            mid_jump: false,
            last_stood_on: Vec2::Y,
            stood_on_potential: 0.0,
        }
    }
}

struct PlayerMovementSettings {
    pub max_speed: f32,
    pub impulse_exponent: f32,
    pub impulse_coefficient: f32,
    pub jump_power_coefficient: f32,
    pub jump_brake_coefficient: f32,
    pub start_fall_before_peak: f32,
    pub start_of_fall_range: f32,
    pub start_of_fall_gravity_boost: f32,
    pub fall_boost_coefficient: f32,
    pub stood_on_time_coefficient: f32,
    pub uphill_move_exponent: f32,
    pub downhill_brake_exponent: f32,
}

fn control_player(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(
        Entity,
        &mut Velocity,
        &mut PlayerControl,
    )>,
    player_movement_settings: Res<PlayerMovementSettings>,
    rapier_context: Res<RapierContext>,
) {
    let is_jumping = input.pressed(KeyCode::Up);
    let mut target_speed: f32 = 0.0;
    if input.pressed(KeyCode::Left) {
        target_speed -= 1.0;
    }
    if input.pressed(KeyCode::Right) {
        target_speed += 1.0;
    }
    for (player_entity, mut velocity, mut player_control) in query.iter_mut() {
        let standing_on = rapier_context
            .contacts_with(player_entity)
            .filter(|contact| contact.raw.has_any_active_contact)
            .filter_map(|contact| {
                contact.manifolds().filter_map(|contact_manifold| {
                    if contact.collider1() == player_entity {
                        Some(-contact_manifold.normal())
                    } else if contact.collider2() == player_entity {
                        Some(contact_manifold.normal())
                    } else {
                        None
                    }
                }).max_by_key(|normal| float_ord::FloatOrd(normal.dot(Vec2::Y)))
            })
            .max_by_key(|normal| float_ord::FloatOrd(normal.dot(Vec2::Y)));

        enum JumpStatus {
            CanJump,
            InitiateJump,
            GoingUp,
            StoppingUp,
            GoingDown,
        }

        let jump_status = (|| {
            if let Some(standing_on) = standing_on {
                player_control.last_stood_on = standing_on;
                player_control.stood_on_potential = 1.0;
                if 0.0 < standing_on.dot(Vec2::Y) {
                    if is_jumping {
                        return JumpStatus::InitiateJump;
                    }
                    return JumpStatus::CanJump;
                }
            }
            player_control.stood_on_potential = (player_control.stood_on_potential
                - time.delta().as_secs_f32() * player_movement_settings.stood_on_time_coefficient)
                .max(0.0);

            if 0.0 <= velocity.linvel.y {
                if is_jumping && player_control.mid_jump {
                    JumpStatus::GoingUp
                } else {
                    JumpStatus::StoppingUp
                }
            } else {
                JumpStatus::GoingDown
            }
        })();

        match jump_status {
            JumpStatus::CanJump => {
                player_control.mid_jump = false;
            }
            JumpStatus::InitiateJump => {
                player_control.mid_jump = true;
                velocity.linvel += Vec2::Y * player_movement_settings.jump_power_coefficient;
            }
            JumpStatus::GoingUp => {
                player_control.mid_jump = true;
            }
            JumpStatus::StoppingUp => {
                player_control.mid_jump = false;
                velocity.linvel.y *= player_movement_settings
                    .jump_brake_coefficient
                    .powf(time.delta().as_secs_f32());
                if velocity.linvel.y < player_movement_settings.start_fall_before_peak {
                    velocity.linvel.y -= player_movement_settings.start_of_fall_gravity_boost
                        * time.delta().as_secs_f32();
                }
            }
            JumpStatus::GoingDown => {
                if -player_movement_settings.start_of_fall_range < velocity.linvel.y {
                    // reminder: linvel.y is negative here
                    velocity.linvel.y -= player_movement_settings.start_of_fall_gravity_boost
                        * time.delta().as_secs_f32();
                } else {
                    velocity.linvel.y *= player_movement_settings
                        .fall_boost_coefficient
                        .powf(time.delta().as_secs_f32());
                }
                player_control.mid_jump = false;
            }
        }

        let mut up_now = Vec2::new(0.0, 1.0);
        up_now = (1.0 - player_control.stood_on_potential) * up_now
            + player_control.stood_on_potential * player_control.last_stood_on;

        let movement_vector = -up_now.perp();

        let current_speed =
            velocity.linvel.dot(movement_vector) / player_movement_settings.max_speed;

        if (0.0 < target_speed && target_speed <= current_speed)
            || (target_speed < 0.0 && current_speed <= target_speed)
        {
            continue;
        }
        let impulse = target_speed - current_speed;
        let impulse = if 1.0 < impulse.abs() {
            impulse.signum()
        } else {
            impulse.signum()
                * impulse
                    .abs()
                    .powf(player_movement_settings.impulse_exponent)
        };
        let mut impulse = movement_vector
            * time.delta().as_secs_f32()
            * player_movement_settings.impulse_coefficient
            * impulse;
        let uphill = impulse.normalize().dot(Vec2::Y);
        if 0.01 <= uphill {
            let efficiency = if target_speed.signum() as i32 == current_speed.signum() as i32 {
                player_movement_settings.uphill_move_exponent
            } else {
                player_movement_settings.downhill_brake_exponent
            };
            impulse *= 1.0 - uphill.powf(efficiency);
        }
        velocity.linvel += impulse;
    }
}
