mod animation_helpers;
mod arena;
mod camera;
mod crystal;
mod doofus;
mod door;
mod gate;
mod global_types;
mod ina;
mod input;
mod level_progress;
mod loading;
mod menu;
mod player_control;
mod utils;
mod yoleck_utils;

use crate::loading::LoadingPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;
use bevy_yoleck::{
    YoleckEditorState, YoleckLoadingCommand, YoleckManaged, YoleckSource, YoleckTypeHandlers,
};

use self::animation_helpers::AnimationHelpersPlugin;
use self::camera::CameraPlugin;
use self::crystal::CrystalPlugin;
use self::doofus::DoofusPlugin;
use self::door::DoorPlugin;
use self::gate::GatePlugin;
pub use self::global_types::MenuActionForKbgp;
use self::global_types::{AppState, LevelProgress, MenuState};
use self::ina::InaPlugin;
use self::input::GameInputPlugin;
use self::level_progress::LevelProgressPlugin;
use self::menu::MenuPlugin;
use self::player_control::PlayerControlPlugin;

pub struct GamePlugin {
    pub is_editor: bool,
    pub start_at_level: Option<String>,
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
            door::Door::handler("Door"),
            gate::Gate::handler("Gate"),
            crystal::Crystal::handler("Crystal"),
        ]));
        if !self.is_editor {
            app.add_plugin(MenuPlugin);
        }
        app.add_plugin(LoadingPlugin);
        app.add_plugin(CameraPlugin);
        app.add_plugin(AnimationHelpersPlugin);
        app.add_plugin(DoofusPlugin);
        app.add_plugin(InaPlugin);
        app.add_plugin(PlayerControlPlugin);
        app.add_plugin(GameInputPlugin);
        app.add_plugin(DoorPlugin);
        app.add_plugin(CrystalPlugin);
        app.add_plugin(GatePlugin);
        app.add_plugin(LevelProgressPlugin);
        app.add_system(enable_disable_physics);
        if self.is_editor {
            app.add_system(set_app_state_based_on_editor_state);
        } else {
            app.add_system_set(
                SystemSet::on_enter(AppState::LoadLevel).with_system(handle_level_loading),
            );
            if let Some(start_at_level) = &self.start_at_level {
                let start_at_level = format!("{}.yol", start_at_level);
                app.add_startup_system(
                    move |mut level_progress: ResMut<LevelProgress>,
                          mut state: ResMut<State<AppState>>| {
                        level_progress.current_level = Some(start_at_level.clone());
                        state.set(AppState::LoadLevel).unwrap();
                    },
                );
            }
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
    level_progress: Res<LevelProgress>,
    mut yoleck_loading_command: ResMut<YoleckLoadingCommand>,
    mut state: ResMut<State<AppState>>,
) {
    for entity in level_entities_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    let current_level = level_progress
        .current_level
        .as_ref()
        .expect("Entered LoadLevel state when current_level is None");
    *yoleck_loading_command =
        YoleckLoadingCommand::FromAsset(asset_server.load(&format!("levels/{}", current_level)));
    state.set(AppState::Game).unwrap();
}
