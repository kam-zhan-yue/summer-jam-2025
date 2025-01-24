use bevy::{asset::AssetMetaCheck, prelude::*};

mod animations;
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
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Swirly Whirly".into(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
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
        .run();
}
