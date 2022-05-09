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
mod menu;

use crate::loading::LoadingPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;
use bevy_yoleck::{YoleckEditorState, YoleckSource, YoleckTypeHandlers, YoleckLoadingCommand, YoleckManaged};

use self::animation_helpers::AnimationHelpersPlugin;
use self::camera::CameraPlugin;
use self::doofus::DoofusPlugin;
use self::global_types::{AppState, MenuState, CurrentLevel};
use self::ina::InaPlugin;
use self::input::GameInputPlugin;
use self::menu::MenuPlugin;
use self::player_control::PlayerControlPlugin;

pub struct GamePlugin {
    pub is_editor: bool,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        if self.is_editor {
            app.add_state(AppState::Editor);
        } else {
            app.add_state(AppState::Menu(MenuState::Main));
        }
        app.insert_resource(YoleckTypeHandlers::new([
            arena::Block::handler("Block"),
            doofus::Doofus::handler("Doofus"),
            ina::Ina::handler("Ina"),
        ]));
        if !self.is_editor {
            app.insert_resource(CurrentLevel(None));
            app.add_plugin(MenuPlugin);
        }
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
            app.add_system_set(SystemSet::on_enter(AppState::LoadLevel).with_system(handle_level_loading));
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
    state: Res<State<AppState>>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
) {
    rapier_configuration.physics_pipeline_active = *state.current() == AppState::Game;
}

fn handle_level_loading(
    level_entities_query: Query<Entity, With<YoleckManaged>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_level: Res<CurrentLevel>,
    mut yoleck_loading_command: ResMut<YoleckLoadingCommand>,
    mut state: ResMut<State<AppState>>,
) {
    for entity in level_entities_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    let current_level = current_level.0.as_ref().expect("Entered LoadLevel state when current_level is None");
    *yoleck_loading_command = YoleckLoadingCommand::FromAsset(asset_server.load(current_level));
    state.set(AppState::Game).unwrap();
}
