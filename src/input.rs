use bevy::prelude::*;
use ezinput::prelude::*;

use crate::global_types::InputBinding;

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EZInputPlugin::<InputBinding>::default());
        app.init_resource::<InputConfig>();
        app.add_startup_system(setup_keyboard_input);
        app.add_system(handle_gamepad_events);
    }
}

struct InputConfig(InputView<InputBinding>);

impl Default for InputConfig {
    fn default() -> Self {
        let mut view = InputView::empty();

        let mut binding = ActionBinding::from(InputBinding::MoveHorizontal);
        for (input, axis_value) in [
            (BindingInputReceiver::KeyboardKey(KeyCode::Left), -1.0),
            (BindingInputReceiver::KeyboardKey(KeyCode::A), -1.0),
            (BindingInputReceiver::KeyboardKey(KeyCode::Right), 1.0),
            (BindingInputReceiver::KeyboardKey(KeyCode::D), 1.0),
            (
                BindingInputReceiver::GamepadButton(GamepadButtonType::DPadLeft),
                -1.0,
            ),
            (
                BindingInputReceiver::GamepadButton(GamepadButtonType::DPadRight),
                1.0,
            ),
        ] {
            binding
                .receiver(input)
                .default_axis_value(input, axis_value);
        }
        binding.receiver(BindingInputReceiver::GamepadAxis(
            GamepadAxisType::LeftStickX,
        ));
        binding.receiver(BindingInputReceiver::GamepadAxis(GamepadAxisType::DPadX));
        view.add_binding(&binding);

        view.add_binding({
            ActionBinding::from(InputBinding::Jump)
                .receiver(BindingInputReceiver::KeyboardKey(KeyCode::Up))
                .receiver(BindingInputReceiver::KeyboardKey(KeyCode::W))
                .receiver(BindingInputReceiver::GamepadButton(
                    GamepadButtonType::South,
                ))
        });

        view.add_binding({
            ActionBinding::from(InputBinding::Pause)
                .receiver(BindingInputReceiver::KeyboardKey(KeyCode::Escape))
                .receiver(BindingInputReceiver::GamepadButton(
                    GamepadButtonType::Start,
                ))
        });

        Self(view)
    }
}

fn setup_keyboard_input(mut commands: Commands, input_config: Res<InputConfig>) {
    commands
        .spawn()
        .insert(input_config.0.clone())
        .insert(EZInputKeyboardService);
}

fn handle_gamepad_events(
    mut reader: EventReader<GamepadEvent>,
    gamepad_services: Query<(Entity, &EZInputGamepadService), With<InputView<InputBinding>>>,
    mut commands: Commands,
    input_config: Res<InputConfig>,
) {
    for GamepadEvent(gamepad, event_type) in reader.iter() {
        match event_type {
            GamepadEventType::Connected => {
                if !gamepad_services
                    .iter()
                    .any(|(_, service)| service.0 == *gamepad)
                {
                    commands
                        .spawn()
                        .insert(input_config.0.clone())
                        .insert(EZInputGamepadService(*gamepad));
                }
            }
            GamepadEventType::Disconnected => {
                for (entity, service) in gamepad_services.iter() {
                    if service.0 == *gamepad {
                        commands.entity(entity).despawn();
                    }
                }
            }
            _ => {}
        }
    }
}
