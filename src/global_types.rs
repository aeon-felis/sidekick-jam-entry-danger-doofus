#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub enum AppState {
    Menu(MenuState),
    // ClearLevelAndThenLoad,
    // LoadLevel,
    // Game,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub enum MenuState {
    Main,
    // Pause,
    // GameOver,
}
