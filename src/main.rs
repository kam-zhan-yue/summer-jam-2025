use bevy::prelude::*;

mod camera;
mod combo;
mod rhythm;
mod schedule;

use camera::CameraPlugin;
use combo::ComboPlugin;
use rhythm::RhythmPlugin;
use schedule::SchedulePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RhythmPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(ComboPlugin)
        .add_plugins(SchedulePlugin)
        .run();
}
