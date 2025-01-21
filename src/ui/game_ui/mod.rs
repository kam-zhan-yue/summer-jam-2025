mod choices;
mod effects;
mod end_turn;
mod resolve;
mod timer;
mod title;
mod ui_assets;

use choices::ChoicesPlugin;
use effects::EffectsPlugin;
use end_turn::EndTurnPlugin;
use resolve::ResolvePlugin;
use timer::TimerPlugin;
use title::TitlePlugin;
use ui_assets::UiAssetsPlugin;

use bevy::prelude::*;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TimerPlugin);
        app.add_plugins(ChoicesPlugin);
        app.add_plugins(TitlePlugin);
        app.add_plugins(ResolvePlugin);
        app.add_plugins(EndTurnPlugin);
        app.add_plugins(EffectsPlugin);
        app.add_plugins(UiAssetsPlugin);
    }
}
