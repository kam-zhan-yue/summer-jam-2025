use crate::schedule::GameSet;
use bevy::prelude::*;

const BEAT_LIMIT: i32 = 2;
const BEAT_TIME: f32 = 0.5;

#[derive(Resource, Debug)]
pub struct Rhythm {
    timer: Timer,
    beat: i32,
}

impl Default for Rhythm {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(BEAT_TIME, TimerMode::Repeating),
            beat: 0,
        }
    }
}

pub struct RhythmPlugin;

impl Plugin for RhythmPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Rhythm>();
        app.add_systems(Startup, setup_rhythm);
        app.add_systems(Update, update_rhythm.in_set(GameSet::Rhythm));
    }
}

fn setup_rhythm() {}

fn update_rhythm(time: Res<Time>, mut rhythm: ResMut<Rhythm>) {
    rhythm.timer.tick(time.delta());
    if rhythm.timer.just_finished() {
        rhythm.beat += 1;
        if rhythm.beat == BEAT_LIMIT + 1 {
            rhythm.beat = 0;
            println!("Resolving!");
        } else {
            println!("Beat is {:?}", rhythm.beat);
        }
    }
}
