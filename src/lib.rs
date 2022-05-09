mod animation_helpers;
mod arena;
mod camera;
mod doofus;
mod global_types;
mod ina;
mod input;
mod loading;
mod player_control;
mod utils;
mod yoleck_utils;

use crate::loading::LoadingPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;
use bevy_yoleck::{YoleckEditorState, YoleckLoadingCommand, YoleckSource, YoleckTypeHandlers};

use self::animation_helpers::AnimationHelpersPlugin;
use self::camera::CameraPlugin;
use self::doofus::DoofusPlugin;
use self::global_types::{AppState, MenuState};
use self::ina::InaPlugin;
use self::input::GameInputPlugin;
use self::player_control::PlayerControlPlugin;

pub struct GamePlugin {
    pub is_editor: bool,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        if false {
            app.add_state(AppState::Menu(MenuState::Main));
        }
        if self.is_editor {
            app.add_state(AppState::Editor);
        } else {
            app.add_state(AppState::Game);
        }
        app.insert_resource(YoleckTypeHandlers::new([
            arena::Block::handler("Block"),
            doofus::Doofus::handler("Doofus"),
            ina::Ina::handler("Ina"),
        ]));
        app.add_plugin(LoadingPlugin);
        app.add_plugin(CameraPlugin);
        app.add_plugin(AnimationHelpersPlugin);
        app.add_plugin(DoofusPlugin);
        app.add_plugin(InaPlugin);
        app.add_plugin(PlayerControlPlugin);
        app.add_plugin(GameInputPlugin);
        app.add_system(enable_disable_physics);
        if self.is_editor {
            app.add_system(set_app_state_based_on_editor_state);
        } else {
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

fn set_app_state_based_on_editor_state(
    yoleck_editor_state: Res<State<YoleckEditorState>>,
    mut app_state: ResMut<State<AppState>>,
) {
    let _ = app_state.set(match yoleck_editor_state.current() {
        YoleckEditorState::EditorActive => AppState::Editor,
        YoleckEditorState::GameActive => AppState::Game,
    });
}

fn enable_disable_physics(
    yoleck_editor_state: Res<State<YoleckEditorState>>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
) {
    rapier_configuration.physics_pipeline_active =
        matches!(yoleck_editor_state.current(), YoleckEditorState::GameActive);
}
