use bevy::prelude::*;

#[derive(Debug, Default, States, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Start,
    Game,
    GameOver,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
    }
}
