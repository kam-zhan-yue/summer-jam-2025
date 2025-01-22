use bevy::prelude::*;

#[derive(Debug, Default, States, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
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
    Title,
    Countdown,
    Reveal,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.init_state::<UiState>();
    }
}
