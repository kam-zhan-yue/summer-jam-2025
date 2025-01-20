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

impl Default for Rhythm {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(BEAT_TIME, TimerMode::Once),
            beat: 0,
        }
    }
}

#[derive(Event, Debug)]
pub struct BeatEvent(pub i32);

impl BeatEvent {
    pub fn new(beat: i32) -> Self {
        BeatEvent(beat)
    }
}

#[derive(Event, Debug, Default)]
pub struct ResolveEvent;

pub struct RhythmPlugin;

impl Plugin for RhythmPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Rhythm>();
        app.add_event::<BeatEvent>();
        app.add_event::<ResolveEvent>();
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
    mut beat_event_writer: EventWriter<BeatEvent>,
    mut resolve_event_writer: EventWriter<ResolveEvent>,
    mut game_flow: ResMut<NextState<GameFlow>>,
) {
    rhythm.timer.tick(time.delta());
    if rhythm.timer.just_finished() {
        rhythm.beat += 1;
        if rhythm.beat == BEAT_LIMIT + 1 {
            rhythm.beat = 0;
            resolve_event_writer.send(ResolveEvent::default());
        } else {
            beat_event_writer.send(BeatEvent::new(rhythm.beat));
        }
        game_flow.set(GameFlow::Reveal);
    }
}
