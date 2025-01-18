use crate::schedule::GameSet;
use bevy::prelude::*;

const BEAT_LIMIT: i32 = 2;
const BEAT_TIME: f32 = 1.;

#[derive(Resource, Debug)]
pub struct Rhythm {
    timer: Timer,
    pub beat: i32,
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
        app.init_resource::<Rhythm>()
            .add_systems(Startup, setup_rhythm)
            .add_systems(Update, update_rhythm.in_set(GameSet::Rhythm))
            .add_event::<BeatEvent>()
            .add_event::<ResolveEvent>();
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

fn setup_rhythm() {}

fn update_rhythm(
    time: Res<Time>,
    mut rhythm: ResMut<Rhythm>,
    mut beat_event_writer: EventWriter<BeatEvent>,
    mut resolve_event_writer: EventWriter<ResolveEvent>,
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
    }
}
