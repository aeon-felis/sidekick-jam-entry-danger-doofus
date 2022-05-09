use bevy::prelude::*;
use ezinput::prelude::BindingTypeView;
use ezinput_macros::BindingTypeView;
use serde::{Deserialize, Serialize};

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub enum AppState {
    Menu(MenuState),
    // ClearLevelAndThenLoad,
    // LoadLevel,
    Game,
    Editor,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub enum MenuState {
    Main,
    // Pause,
    // GameOver,
}

#[derive(BindingTypeView, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum InputBinding {
    MoveHorizontal,
    Jump,
    Pause,
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
pub struct IsPlatform;
