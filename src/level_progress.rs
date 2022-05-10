use bevy::prelude::*;
use bevy_yoleck::YoleckLevelIndex;

use crate::global_types::{AppState, LevelProgress, MenuState};
use crate::utils::some_or;

pub struct LevelProgressPlugin;

impl Plugin for LevelProgressPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelProgress {
            just_completed: None,
            current_level: None,
            level_index_handle: Default::default(),
        });
        app.add_startup_system(setup_level_progress);
        app.add_system_set(
            SystemSet::on_update(AppState::LevelCompleted).with_system(handle_level_completion),
        );
    }
}

fn setup_level_progress(asset_server: Res<AssetServer>, mut level_progress: ResMut<LevelProgress>) {
    level_progress.level_index_handle = asset_server.load("levels/index.yoli");
}

fn handle_level_completion(
    level_index_assets: Res<Assets<YoleckLevelIndex>>,
    mut level_progress: ResMut<LevelProgress>,
    mut state: ResMut<State<AppState>>,
) {
    let level_index = some_or!(level_index_assets.get(&level_progress.level_index_handle); return);
    let mut it = level_index.iter();
    let _current_level = it
        .by_ref()
        .find(|level| level.filename == *level_progress.current_level.as_ref().unwrap())
        .expect("Current level must be in the index");
    level_progress.just_completed = level_progress.current_level.take();
    level_progress.current_level = it.next().map(|level| level.filename.clone());
    state
        .set(AppState::Menu(MenuState::LevelCompleted))
        .unwrap();
}
