use bevy::prelude::*;

#[derive(Debug, Default, States, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Start,
    Game,
    GameOver,
}

#[derive(Debug, Default, States, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameFlow {
    #[default]
    None,
    Title,
    Countdown,
    Reveal,

    RoundStart,
    SelectElement,
    SelectAction,
    ResolveAction,
    ComboBreaker,
    RoundOver,
}

#[derive(Debug, Default, States, Clone, Copy, Eq, PartialEq, Hash)]
pub enum UiFlow {
    #[default]
    Title,
    Countdown,
    Reveal,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.init_state::<GameFlow>();
        app.init_state::<UiFlow>();
    }
}
