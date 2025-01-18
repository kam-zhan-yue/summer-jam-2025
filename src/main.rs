use bevy::prelude::*;

mod camera;
mod combo;
mod rhythm;
mod schedule;
mod state;
mod types;

use camera::CameraPlugin;
use combo::ComboPlugin;
use rhythm::RhythmPlugin;
use schedule::SchedulePlugin;
use state::StatePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RhythmPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(ComboPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin)
        .run();
}
