pub enum GameState {
    MainMenu,
    Settings,
    InGame(InGameState),
}
pub enum InGameState {
    Paused,
    Playing,
}
