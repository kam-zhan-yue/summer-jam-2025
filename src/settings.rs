use bevy::prelude::*;

#[derive(Debug, Default)]
pub enum GameMode {
    #[default]
    SinglePlayer,
    TwoPlayer,
}

#[derive(Resource, Debug, Default)]
pub struct GameSettings {
    game_mode: GameMode,
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSettings>();
    }
}
