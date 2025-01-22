mod game_ui;
mod main_menu;

use bevy::prelude::*;
use game_ui::GameUIPlugin;
use main_menu::MainMenuPlugin;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainMenuPlugin);
        app.add_plugins(GameUIPlugin);
    }
}
