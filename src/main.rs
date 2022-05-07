// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{App, ClearColor, Color, Msaa, WindowDescriptor};
use bevy::DefaultPlugins;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use danger_doofus::GamePlugin;

fn main() {
    let is_editor = std::env::args().any(|arg| arg == "--editor");
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
    app.add_plugin(bevy_yoleck::bevy_egui::EguiPlugin);
    if is_editor {
        app.add_plugin(bevy_yoleck::YoleckPluginForEditor);
        app.add_plugin(bevy_yoleck::tools_2d::YoleckTools2dPlugin);
    } else {
        app.add_plugin(bevy_yoleck::YoleckPluginForGame);
    }
    app.add_plugin(GamePlugin { is_editor });
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0));
    app.run();
}
