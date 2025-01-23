use bevy::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod camera;
mod combo;
mod computer;
mod config;
mod events;
mod flow;
mod globals;
mod helper;
mod schedule;
mod settings;
mod state;
mod types;
mod ui;

use bevy_tweening::TweeningPlugin;
use camera::CameraPlugin;
use combo::ComboPlugin;
use computer::ComputerPlugin;
use events::EventsPlugin;
use flow::FlowPlugin;
use globals::GlobalPlugin;
use schedule::SchedulePlugin;
use settings::SettingsPlugin;
use state::StatePlugin;
use ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(GlobalPlugin)
        .add_plugins(EventsPlugin)
        .add_plugins(FlowPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(ComboPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin)
        .add_plugins(SettingsPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(TweeningPlugin)
        .add_plugins(ComputerPlugin)
        // .add_plugins(WorldInspectorPlugin::new())
        .run();
}
