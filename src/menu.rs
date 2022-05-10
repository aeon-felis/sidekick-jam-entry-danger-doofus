use bevy::prelude::*;
use bevy_egui_kbgp::bevy_egui::EguiContext;
use bevy_egui_kbgp::egui;
use bevy_egui_kbgp::prelude::*;
use bevy_yoleck::YoleckLevelIndex;

use crate::global_types::CurrentLevel;
use crate::global_types::{AppState, MenuState};
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
        AppState::Editor => panic!("Menu and editor must not run together"),
    }
}

#[derive(PartialEq)]
enum FocusLabel {
    Start,
    NextLevel,
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
    mut current_level: ResMut<CurrentLevel>,
    asset_server: Res<AssetServer>,
    level_index_assets: Res<Assets<YoleckLevelIndex>>,
    mut level_index_handle: Local<Option<Handle<YoleckLevelIndex>>>,
) {
    menu_layout(egui_context.ctx_mut(), |ui| {
        if ui.kbgp_user_action() == Some(MenuActionForKbgp) {
            ui.kbgp_set_focus_label(FocusLabel::BackToMainMenu);
        }
        if ui
            .button("Back To Menu")
            .kbgp_navigation()
            .kbgp_focus_label(FocusLabel::Exit)
            .clicked()
        {
            state.set(AppState::Menu(MenuState::Main)).unwrap();
            ui.kbgp_clear_input();
        }
        let handle =
            level_index_handle.get_or_insert_with(|| asset_server.load("levels/index.yoli"));
        let level_index = some_or!(level_index_assets.get(handle.clone()); return);
        for (index, level) in level_index.iter().enumerate() {
            let caption = level
                .filename
                .strip_suffix(".yol")
                .unwrap_or(&level.filename)
                .replace('_', " ");
            let mut response = ui.button(caption).kbgp_navigation();
            if index == level_index.len() - 1 {
                response = response.kbgp_focus_label(FocusLabel::NextLevel);
            }
            if response.clicked() {
                current_level.0 = Some(format!("levels/{}", level.filename));
                state.set(AppState::LoadLevel).unwrap();
            }
        }
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
