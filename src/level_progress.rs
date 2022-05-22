use bevy::prelude::*;
use bevy_pkv::PkvStore;
use bevy_yoleck::YoleckLevelIndex;

use crate::global_types::{AppState, LevelProgress, MenuState};
use crate::loading::GameAssets;
use crate::utils::some_or;

pub struct LevelProgressPlugin;

impl Plugin for LevelProgressPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelProgress {
            just_completed: None,
            current_level: None,
            num_levels_available: 0,
        });
        app.add_system(read_last_finished_level);
        app.add_system_set(
            SystemSet::on_update(AppState::LevelCompleted).with_system(handle_level_completion),
        );
    }
}

const LEVEL_PKV_KEY: &str = "completed_up_to_level";

fn read_last_finished_level(
    pkv: Res<PkvStore>,
    mut level_progress: ResMut<LevelProgress>,
    game_assets: Res<GameAssets>,
    level_index_assets: Res<Assets<YoleckLevelIndex>>,
) {
    if 0 < level_progress.num_levels_available {
        return;
    }
    if let Ok(completed_up_to_level) = pkv.get::<String>(LEVEL_PKV_KEY) {
        let level_index = some_or!(level_index_assets.get(&game_assets.level_index); return);
        if let Some(index) = level_index.iter().enumerate().find_map(|(index, level)| {
            if level.filename == completed_up_to_level {
                Some(index)
            } else {
                None
            }
        }) {
            level_progress.num_levels_available = index + 2;
        } else {
            error!(
                "Unable to find level {:?}, starting anew",
                completed_up_to_level
            );
            level_progress.num_levels_available = 1;
        }
    } else {
        level_progress.num_levels_available = 1;
    }
}

fn handle_level_completion(
    game_assets: Res<GameAssets>,
    level_index_assets: Res<Assets<YoleckLevelIndex>>,
    mut pkv: ResMut<PkvStore>,
    mut level_progress: ResMut<LevelProgress>,
    mut state: ResMut<State<AppState>>,
) {
    let completed_level = some_or!(
        level_progress.current_level.as_ref();
        return // level completed inside editor
    );
    let level_index = some_or!(level_index_assets.get(&game_assets.level_index); return);
    let mut it = level_index.iter();

    let is_new_level_better = if let Ok(best_completed) = pkv.get::<String>(LEVEL_PKV_KEY) {
        level_index
            .iter()
            .rev()
            .find_map(|level| {
                if level.filename == best_completed {
                    Some(false)
                } else if level.filename == *completed_level {
                    Some(true)
                } else {
                    None
                }
            })
            .unwrap_or(true)
    } else {
        true
    };

    if is_new_level_better {
        if let Err(err) = pkv.set(LEVEL_PKV_KEY, completed_level) {
            error!("Cannot save level progression: {}", err);
        }
    }
    let _current_level = it
        .by_ref()
        .find(|level| level.filename == *completed_level)
        .expect("Current level must be in the index");
    level_progress.just_completed = level_progress.current_level.take();
    level_progress.current_level = it.next().map(|level| level.filename.clone());
    level_progress.num_levels_available = 0;
    state
        .set(AppState::Menu(MenuState::LevelCompleted))
        .unwrap();
}
