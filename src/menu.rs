use bevy::prelude::*;
use bevy_egui_kbgp::bevy_egui::EguiContext;
use bevy_egui_kbgp::egui;
use bevy_egui_kbgp::prelude::*;
use bevy_yoleck::YoleckLevelIndex;

use crate::global_types::LevelProgress;
use crate::global_types::{AppState, MenuState};
use crate::loading::GameAssets;
use crate::utils::some_or;
use crate::MenuActionForKbgp;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(pause_unpause_game);
        app.add_system_set(
            SystemSet::on_update(AppState::Menu(MenuState::Main)).with_system(main_menu),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Menu(MenuState::LevelSelect))
                .with_system(level_select_menu),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Menu(MenuState::Pause)).with_system(pause_menu),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Menu(MenuState::LevelCompleted))
                .with_system(level_completed_menu),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Menu(MenuState::GameOver)).with_system(game_over_menu),
        );
    }
}

fn pause_unpause_game(mut egui_context: ResMut<EguiContext>, mut state: ResMut<State<AppState>>) {
    match state.current() {
        AppState::Menu(_) => {}
        // AppState::ClearLevelAndThenLoad => {}
        AppState::LoadLevel => {}
        AppState::Game => {
            let egui_context = egui_context.ctx_mut();
            if egui_context.kbgp_user_action() == Some(MenuActionForKbgp) {
                state.set(AppState::Menu(MenuState::Pause)).unwrap();
                egui_context.kbgp_clear_input();
            }
        }
        AppState::LevelCompleted => {}
        AppState::Editor => panic!("Menu and editor must not run together"),
    }
}

#[derive(PartialEq)]
enum FocusLabel {
    Start,
    NextLevel,
    CurrentLevel,
    BackToMainMenu,
    Exit,
}

fn menu_layout(egui_context: &egui::Context, dlg: impl FnOnce(&mut egui::Ui)) {
    egui::CentralPanel::default()
        .frame(egui::Frame::none())
        .show(egui_context, |ui| {
            let layout = egui::Layout::top_down(egui::Align::Center);
            ui.with_layout(layout, |ui| {
                dlg(ui);
            });
        });
}

fn format_level_name(filename: &str) -> String {
    filename
        .strip_suffix(".yol")
        .unwrap_or(filename)
        .replace('_', " ")
}

fn main_menu(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
    #[cfg(not(target_arch = "wasm32"))] mut exit: EventWriter<bevy::app::AppExit>,
) {
    menu_layout(egui_context.ctx_mut(), |ui| {
        if ui.kbgp_user_action() == Some(MenuActionForKbgp) {
            ui.kbgp_set_focus_label(FocusLabel::Exit);
        }
        if ui
            .button("Start")
            .kbgp_navigation()
            .kbgp_initial_focus()
            .kbgp_focus_label(FocusLabel::Start)
            .clicked()
        {
            state.set(AppState::Menu(MenuState::LevelSelect)).unwrap();
            ui.kbgp_clear_input();
            ui.kbgp_set_focus_label(FocusLabel::NextLevel);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if ui
            .button("Exit")
            .kbgp_navigation()
            .kbgp_focus_label(FocusLabel::Exit)
            .clicked()
        {
            exit.send(bevy::app::AppExit);
        }
    });
}

fn level_select_menu(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
    mut level_progress: ResMut<LevelProgress>,
    game_assets: Res<GameAssets>,
    level_index_assets: Res<Assets<YoleckLevelIndex>>,
) {
    menu_layout(egui_context.ctx_mut(), |ui| {
        if ui.kbgp_user_action() == Some(MenuActionForKbgp) {
            ui.kbgp_set_focus_label(FocusLabel::BackToMainMenu);
        }
        let mut response = ui
            .button("Back To Menu")
            .kbgp_navigation()
            .kbgp_focus_label(FocusLabel::BackToMainMenu);
        let level_index = level_index_assets.get(&game_assets.level_index);
        if level_index
            .map(|level_index| level_index.len() < level_progress.num_levels_available)
            .unwrap_or(true)
        {
            response = response.kbgp_focus_label(FocusLabel::NextLevel);
        }
        if response.clicked() {
            state.set(AppState::Menu(MenuState::Main)).unwrap();
            ui.kbgp_clear_input();
        }
        egui::ScrollArea::vertical().show(ui, |ui| {
            let level_index = some_or!(level_index; return);
            for (index, level) in level_index.iter().enumerate() {
                let mut response = ui
                    .add_enabled(
                        index < level_progress.num_levels_available,
                        egui::Button::new(format_level_name(&level.filename)),
                    )
                    .kbgp_navigation();
                if index + 1 == level_progress.num_levels_available {
                    response = response.kbgp_focus_label(FocusLabel::NextLevel);
                }
                if Some(&level.filename) == level_progress.current_level.as_ref() {
                    response = response.kbgp_focus_label(FocusLabel::CurrentLevel);
                }
                if response.clicked() {
                    level_progress.current_level = Some(level.filename.clone());
                    state.set(AppState::LoadLevel).unwrap();
                }
            }
        });
    });
}

fn pause_menu(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
    #[cfg(not(target_arch = "wasm32"))] mut exit: EventWriter<bevy::app::AppExit>,
) {
    menu_layout(egui_context.ctx_mut(), |ui| {
        if ui
            .button("Resume")
            .kbgp_navigation()
            .kbgp_initial_focus()
            .clicked()
            || ui.kbgp_user_action() == Some(MenuActionForKbgp)
        {
            state.set(AppState::Game).unwrap();
        }
        if ui.button("Retry").kbgp_navigation().clicked() {
            state.set(AppState::LoadLevel).unwrap();
        }
        if ui
            .button("Level Select")
            .kbgp_navigation()
            .kbgp_initial_focus()
            .clicked()
        {
            state.set(AppState::Menu(MenuState::LevelSelect)).unwrap();
            ui.kbgp_clear_input();
            ui.kbgp_set_focus_label(FocusLabel::CurrentLevel);
        }
        if ui.button("Main Menu").kbgp_navigation().clicked() {
            state.set(AppState::Menu(MenuState::Main)).unwrap();
            ui.kbgp_clear_input();
            ui.kbgp_set_focus_label(FocusLabel::Start);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if ui
            .button("Exit")
            .kbgp_navigation()
            .kbgp_focus_label(FocusLabel::BackToMainMenu)
            .clicked()
        {
            exit.send(bevy::app::AppExit);
        }
    });
}

fn level_completed_menu(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
    level_progress: Res<LevelProgress>,
    #[cfg(not(target_arch = "wasm32"))] mut exit: EventWriter<bevy::app::AppExit>,
) {
    menu_layout(egui_context.ctx_mut(), |ui| {
        if ui.kbgp_user_action() == Some(MenuActionForKbgp) {
            ui.kbgp_set_focus_label(FocusLabel::Exit);
        }
        let just_completed = level_progress.just_completed.as_ref().unwrap();
        ui.label(
            egui::RichText::new(format!("Finished {:?}", format_level_name(just_completed)))
                .color(egui::Color32::WHITE)
                .background_color(egui::Color32::BLACK)
                .text_style(egui::TextStyle::Heading),
        );
        ui.add_space(8.0);
        if let Some(current_level) = &level_progress.current_level {
            if ui
                .button(format!("Next Level: {}", format_level_name(current_level)))
                .kbgp_navigation()
                .kbgp_initial_focus()
                .clicked()
            {
                state.set(AppState::LoadLevel).unwrap();
            }
        } else {
            ui.label(
                egui::RichText::new("Congratulations!\nYou have finished the game!")
                    .strong()
                    .color(egui::Color32::WHITE)
                    .background_color(egui::Color32::BLACK)
                    .text_style(egui::TextStyle::Heading),
            );
            ui.add_space(8.0);
        }
        if ui
            .button("Level Select")
            .kbgp_navigation()
            .kbgp_initial_focus()
            .clicked()
        {
            state.set(AppState::Menu(MenuState::LevelSelect)).unwrap();
            ui.kbgp_clear_input();
            if level_progress.current_level.is_some() {
                ui.kbgp_set_focus_label(FocusLabel::CurrentLevel);
            } else {
                ui.kbgp_set_focus_label(FocusLabel::BackToMainMenu);
            }
        }
        if ui.button("Main Menu").kbgp_navigation().clicked() {
            state.set(AppState::Menu(MenuState::Main)).unwrap();
            ui.kbgp_clear_input();
            ui.kbgp_set_focus_label(FocusLabel::Start);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if ui
            .button("Exit")
            .kbgp_navigation()
            .kbgp_focus_label(FocusLabel::Exit)
            .clicked()
        {
            exit.send(bevy::app::AppExit);
        }
    });
}

fn game_over_menu(
    mut egui_context: ResMut<EguiContext>,
    mut state: ResMut<State<AppState>>,
    #[cfg(not(target_arch = "wasm32"))] mut exit: EventWriter<bevy::app::AppExit>,
) {
    menu_layout(egui_context.ctx_mut(), |ui| {
        if ui.kbgp_user_action() == Some(MenuActionForKbgp) {
            ui.kbgp_set_focus_label(FocusLabel::Exit);
        }
        if ui
            .button("Retry")
            .kbgp_navigation()
            .kbgp_initial_focus()
            .clicked()
        {
            state.set(AppState::LoadLevel).unwrap();
        }
        if ui.button("Main Menu").kbgp_navigation().clicked() {
            state.set(AppState::Menu(MenuState::Main)).unwrap();
            ui.kbgp_clear_input();
            ui.kbgp_set_focus_label(FocusLabel::Start);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if ui
            .button("Exit")
            .kbgp_navigation()
            .kbgp_focus_label(FocusLabel::Exit)
            .clicked()
        {
            exit.send(bevy::app::AppExit);
        }
    });
}
