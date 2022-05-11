use bevy::prelude::*;
use bevy_yoleck::{egui, YoleckLevelIndex};
use ezinput::prelude::BindingTypeView;
use ezinput_macros::BindingTypeView;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq)]
pub struct MenuActionForKbgp;

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub enum AppState {
    Menu(MenuState),
    // ClearLevelAndThenLoad,
    LoadLevel,
    Game,
    LevelCompleted,
    Editor,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub enum MenuState {
    Main,
    LevelSelect,
    Pause,
    LevelCompleted,
    GameOver,
}

pub struct LevelProgress {
    pub just_completed: Option<String>,
    pub current_level: Option<String>,
    pub level_index_handle: Handle<YoleckLevelIndex>,
    pub num_levels_available: usize,
}

#[derive(Debug)]
pub enum TweenCompletedCode {
    ExitDoorFinished,
}

#[derive(BindingTypeView, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum InputBinding {
    MoveHorizontal,
    Jump,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component, Serialize, Deserialize)]
pub enum Facing {
    Right,
    Left,
}

impl Default for Facing {
    fn default() -> Self {
        Self::Right
    }
}

impl Facing {
    pub fn signum(&self) -> f32 {
        match self {
            Facing::Right => 1.0,
            Facing::Left => -1.0,
        }
    }

    pub fn reverse(&self) -> Self {
        match self {
            Facing::Right => Self::Left,
            Facing::Left => Self::Right,
        }
    }
}

#[derive(Component)]
pub struct IsDoofus;

#[derive(Component)]
pub struct IsIna;

#[derive(Component)]
pub struct IsCrystalActivator;

#[derive(Component)]
pub struct IsPlatform;

#[derive(Component)]
pub struct IsDoor;

#[derive(Component)]
pub struct IsGate;

#[derive(Component)]
pub struct CrystalState {
    pub num_activators: usize,
}

#[derive(Debug, Clone, Copy, Component, Serialize, Deserialize, PartialEq, Eq)]
pub enum ColorCode {
    Red,
    Green,
    Blue,
    Yellow,
}

impl ColorCode {
    pub const fn size() -> usize {
        4
    }

    pub fn items() -> [Self; Self::size()] {
        [Self::Red, Self::Green, Self::Blue, Self::Yellow]
    }

    pub fn egui_rich_text(&self) -> egui::RichText {
        let (bg, fg) = match self {
            ColorCode::Red => (egui::Color32::RED, egui::Color32::WHITE),
            ColorCode::Green => (egui::Color32::GREEN, egui::Color32::BLACK),
            ColorCode::Blue => (egui::Color32::BLUE, egui::Color32::WHITE),
            ColorCode::Yellow => (egui::Color32::YELLOW, egui::Color32::BLACK),
        };
        egui::RichText::new(self.to_string())
            .background_color(bg)
            .color(fg)
    }

    pub fn bevy_color(&self) -> Color {
        match self {
            ColorCode::Red => Color::RED,
            ColorCode::Green => Color::GREEN,
            ColorCode::Blue => Color::BLUE,
            ColorCode::Yellow => Color::YELLOW,
        }
    }
}

impl core::fmt::Display for ColorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ColorCode::Red => "Red",
            ColorCode::Green => "Green",
            ColorCode::Blue => "Blue",
            ColorCode::Yellow => "Yellow",
        })
    }
}

impl Default for ColorCode {
    fn default() -> Self {
        Self::Red
    }
}
