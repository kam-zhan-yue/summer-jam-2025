mod choices;
mod timer;

use choices::ChoicesPlugin;
use timer::TimerPlugin;

use bevy::prelude::*;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TimerPlugin);
        app.add_plugins(ChoicesPlugin);
    }
}
