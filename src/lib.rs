mod arena;
mod camera;
mod global_types;
mod loading;

use crate::loading::LoadingPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;
use bevy_yoleck::{YoleckEditorState, YoleckLoadingCommand, YoleckSource, YoleckTypeHandlers};

use self::camera::CameraPlugin;
use self::global_types::{AppState, MenuState};

pub struct GamePlugin {
    pub is_editor: bool,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Menu(MenuState::Main));
        app.add_plugin(LoadingPlugin);
        app.add_plugin(CameraPlugin);
        app.insert_resource(YoleckTypeHandlers::new([arena::Floor::handler("Floor")]));
        app.add_system(enable_disable_physics);
        if !self.is_editor {
            app.add_startup_system(
                |asset_server: Res<AssetServer>,
                 mut yoleck_loading_command: ResMut<YoleckLoadingCommand>| {
                    *yoleck_loading_command =
                        YoleckLoadingCommand::FromAsset(asset_server.load("levels/level-01.yol"));
                },
            );
        }
    }
}

fn enable_disable_physics(
    yoleck_editor_state: Res<State<YoleckEditorState>>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
) {
    rapier_configuration.physics_pipeline_active =
        matches!(yoleck_editor_state.current(), YoleckEditorState::GameActive);
}
