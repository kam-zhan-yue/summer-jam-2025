mod systems;

use bevy::prelude::*;
use systems::spawn_main_menu;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_main_menu);
    }
}
