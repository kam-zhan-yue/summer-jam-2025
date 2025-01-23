use bevy::{prelude::*, render::camera::SubCameraView};

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
    commands.spawn((
        Camera2d,
        Camera {
            sub_camera_view: Some(SubCameraView {
                full_size: UVec2::new(SCREEN_X as u32, SCREEN_Y as u32),
                offset: Vec2::new(0.0, 0.0),
                size: UVec2::new(SCREEN_X as u32, SCREEN_Y as u32),
            }),
            order: 1,
            ..default()
        },
    ));
}
