use bevy::prelude::*;
use rand::Rng;

const CAMERA_DECAY_RATE: f32 = 0.9; // Adjust this for smoother or snappier decay
const TRAUMA_DECAY_SPEED: f32 = 0.5; // How fast trauma decays
const TRAUMA_INCREMENT: f32 = 1.0; // Increment of trauma per frame when holding space

// screen_shake parameters, maximum addition by frame not actual maximum overall values
const MAX_ANGLE: f32 = 0.5;
const MAX_OFFSET: f32 = 500.0;

pub struct JuicePlugin;

impl Plugin for JuicePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenShake>();
        app.add_systems(Update, (screen_shake, trigger_shake_on_space));
    }
}

#[derive(Resource, Clone)]
struct ScreenShake {
    max_angle: f32,
    max_offset: f32,
    trauma: f32,
    latest_position: Option<Vec2>,
}

impl Default for ScreenShake {
    fn default() -> Self {
        Self {
            max_angle: 0.0,
            max_offset: 0.0,
            trauma: 0.0,
            latest_position: Some(Vec2::default()),
        }
    }
}

impl ScreenShake {
    fn start_shake(&mut self, max_angle: f32, max_offset: f32, trauma: f32, final_position: Vec2) {
        self.max_angle = max_angle;
        self.max_offset = max_offset;
        self.trauma = trauma.clamp(0.0, 1.0);
        self.latest_position = Some(final_position);
    }
}

fn trigger_shake_on_space(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut screen_shake: ResMut<ScreenShake>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("SHAKE SCREEN");
        let screen_shake_clone = screen_shake.clone();
        screen_shake.start_shake(
            MAX_ANGLE,
            MAX_OFFSET,
            screen_shake_clone.trauma + TRAUMA_INCREMENT * time.delta_secs(),
            Vec2 { x: 0.0, y: 0.0 },
        ); // final_position should be your current player position
    }
}

fn screen_shake(
    time: Res<Time>,
    mut screen_shake: ResMut<ScreenShake>,
    mut query: Query<(&mut Camera, &mut Transform)>,
) {
    let mut rng = rand::thread_rng();
    let shake = screen_shake.trauma * screen_shake.trauma;
    let angle = (screen_shake.max_angle * shake).to_radians() * rng.gen_range(-1.0..1.0);
    let offset_x = screen_shake.max_offset * shake * rng.gen_range(-1.0..1.0);
    let offset_y = screen_shake.max_offset * shake * rng.gen_range(-1.0..1.0);

    if shake > 0.0 {
        for (mut camera, mut transform) in query.iter_mut() {
            // Position
            let sub_view = camera.sub_camera_view.as_mut().unwrap();
            let target = sub_view.offset
                + Vec2 {
                    x: offset_x,
                    y: offset_y,
                };
            sub_view
                .offset
                .smooth_nudge(&target, CAMERA_DECAY_RATE, time.delta_secs());

            // Rotation
            let rotation = Quat::from_rotation_z(angle);
            transform.rotation = transform
                .rotation
                .interpolate_stable(&(transform.rotation.mul_quat(rotation)), CAMERA_DECAY_RATE);
        }
    } else {
        // return camera to the latest position of player (it's fixed in this example case)
        if let Ok((mut camera, mut transform)) = query.get_single_mut() {
            let sub_view = camera.sub_camera_view.as_mut().unwrap();
            let target = screen_shake.latest_position.unwrap();
            sub_view
                .offset
                .smooth_nudge(&target, 1.0, time.delta_secs());
            transform.rotation = transform.rotation.interpolate_stable(&Quat::IDENTITY, 0.1);
        }
    }
    // Decay the trauma over time
    screen_shake.trauma -= TRAUMA_DECAY_SPEED * time.delta_secs();
    screen_shake.trauma = screen_shake.trauma.clamp(0.0, 1.0);
}
