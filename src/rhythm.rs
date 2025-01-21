use crate::schedule::GameSet;
use crate::state::GameFlow;
use bevy::prelude::*;

pub const BEAT_LIMIT: i32 = 2;
const BEAT_TIME: f32 = 3.;

#[derive(Resource, Debug)]
pub struct Rhythm {
    pub timer: Timer,
    pub beat: i32,
}

impl Rhythm {
    pub fn can_end_turn(&self) -> bool {
        self.beat >= BEAT_LIMIT
    }
    pub fn reset(&mut self) {
        self.beat = 0;
    }
}

impl Default for Rhythm {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(BEAT_TIME, TimerMode::Once),
            beat: 0,
        }
    }
}

pub struct RhythmPlugin;

impl Plugin for RhythmPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Rhythm>();
        app.add_systems(OnEnter(GameFlow::Countdown), start_countdown);
        app.add_systems(
            Update,
            update_rhythm
                .in_set(GameSet::Rhythm)
                .run_if(in_state(GameFlow::Countdown)),
        );
    }
}

fn start_countdown(mut rhythm: ResMut<Rhythm>) {
    rhythm.timer = Timer::from_seconds(BEAT_TIME, TimerMode::Once);
}

fn update_rhythm(
    time: Res<Time>,
    mut rhythm: ResMut<Rhythm>,
    mut game_flow: ResMut<NextState<GameFlow>>,
) {
    rhythm.timer.tick(time.delta());
    if rhythm.timer.just_finished() {
        rhythm.beat += 1;
        game_flow.set(GameFlow::Reveal);
    }
}
