use bevy::prelude::*;

pub const SCREEN_X: f32 = 1400.;
pub const SCREEN_Y: f32 = 600.;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.insert_resource(ClearColor(Color::WHITE));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
