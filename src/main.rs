use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod camera;
mod combo;
mod helper;
mod rhythm;
mod schedule;
mod settings;
mod state;
mod types;
mod ui;

use camera::CameraPlugin;
use combo::ComboPlugin;
use rhythm::RhythmPlugin;
use schedule::SchedulePlugin;
use settings::SettingsPlugin;
use state::StatePlugin;
use ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RhythmPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(ComboPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin)
        .add_plugins(SettingsPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
