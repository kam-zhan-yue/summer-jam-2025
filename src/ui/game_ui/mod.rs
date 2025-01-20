mod choices;
mod resolve;
mod timer;
mod title;

use choices::ChoicesPlugin;
use resolve::ResolvePlugin;
use timer::TimerPlugin;
use title::TitlePlugin;

use bevy::prelude::*;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TimerPlugin);
        app.add_plugins(ChoicesPlugin);
        app.add_plugins(TitlePlugin);
        app.add_plugins(ResolvePlugin);
    }
}
