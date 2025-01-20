mod game_over;
mod main_menu;

use bevy::prelude::*;
use game_over::GameOverPlugin;
use main_menu::MainMenuPlugin;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainMenuPlugin);
        app.add_plugins(GameOverPlugin);
    }
}
