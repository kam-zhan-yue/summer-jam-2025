use bevy::prelude::*;

#[derive(Debug, Default, States, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    None,
    Title,

    GameStart,
    SelectElement,
    SelectAction,
    ResolveAction,
    GameOver,
}

#[derive(Debug, Default, States, Clone, Copy, Eq, PartialEq, Hash)]
pub enum UiState {
    #[default]
    None,
    Title,
    Countdown,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.init_state::<UiState>();
    }
}
