// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_egui_kbgp::KbgpNavCommand;
use bevy_egui_kbgp::{KbgpNavBindings, KbgpPlugin, KbgpSettings};
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_tweening::TweeningPlugin;
use clap::Parser;
use danger_doofus::GamePlugin;
use danger_doofus::MenuActionForKbgp;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    editor: bool,
    #[clap(long)]
    level: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut app = App::new();
    app.insert_resource(Msaa { samples: 1 });
    app.insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)));
    app.insert_resource(WindowDescriptor {
        width: 800.,
        height: 600.,
        title: "Danger Doofus".to_string(),
        ..Default::default()
    });
    app.add_plugins(DefaultPlugins);
    app.add_plugin(TweeningPlugin);
    app.add_plugin(bevy_yoleck::bevy_egui::EguiPlugin);
    if args.editor {
        app.add_plugin(bevy_yoleck::YoleckPluginForEditor);
        app.add_plugin(bevy_yoleck::tools_2d::YoleckTools2dPlugin);
    } else {
        app.add_plugin(bevy_yoleck::YoleckPluginForGame);
        app.insert_resource(bevy_egui::EguiSettings {
            scale_factor: 2.0,
            default_open_url_target: None,
        });
        app.add_plugin(KbgpPlugin);
        app.insert_resource(KbgpSettings {
            disable_default_navigation: true,
            disable_default_activation: false,
            prevent_loss_of_focus: true,
            focus_on_mouse_movement: true,
            allow_keyboard: true,
            allow_mouse_buttons: false,
            allow_mouse_wheel: false,
            allow_mouse_wheel_sideways: false,
            allow_gamepads: true,
            bindings: {
                KbgpNavBindings::default()
                    .with_wasd_navigation()
                    .with_key(KeyCode::Escape, KbgpNavCommand::user(MenuActionForKbgp))
                    .with_gamepad_button(
                        GamepadButtonType::Start,
                        KbgpNavCommand::user(MenuActionForKbgp),
                    )
            },
        });
    }
    app.add_plugin(GamePlugin {
        is_editor: args.editor,
        start_at_level: args.level,
    });
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0));
    app.run();
}
