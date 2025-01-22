use bevy::prelude::*;
use std::time::Duration;

mod combo_breaker;
mod countdown;
mod resolve_action;
mod round_over;
mod round_start;
mod select_action;
mod select_element;
mod title;

use combo_breaker::ComboBreakerPlugin;
use countdown::CountdownPlugin;
use resolve_action::ResolveActionPlugin;
use round_over::RoundOverPlugin;
use round_start::RoundStartPlugin;
use select_action::SelectActionPlugin;
use select_element::SelectElementPlugin;

use crate::{schedule::GameSet, state::GameFlow};

const TITLE_TIME: f32 = 1.0;
const COUNTDOWN_TIME: f32 = 2.0;
const REVEAL_TIME: f32 = 2.0;
const ROUND_START_TIME: f32 = 1.0;
const SELECT_ELEMENT_TIME: f32 = 3.0;
const SELECT_ACTION_TIME: f32 = 3.0;

#[derive(Resource)]
pub struct Flow {
    pub timer: Timer,
}

impl Flow {
    fn reset(&mut self, timer: Timer) {
        self.timer = timer;
    }

    fn tick(&mut self, duration: Duration) {
        self.timer.tick(duration);
    }
}

impl Default for Flow {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.0, TimerMode::Once),
        }
    }
}

pub struct FlowPlugin;

impl Plugin for FlowPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Flow>();
        app.add_plugins(RoundStartPlugin);
        app.add_plugins(SelectElementPlugin);
        app.add_plugins(SelectActionPlugin);
        app.add_plugins(ResolveActionPlugin);
        app.add_plugins(ComboBreakerPlugin);
        app.add_plugins(RoundOverPlugin);
        app.add_plugins(CountdownPlugin);
        app.add_systems(Update, handle_flow.in_set(GameSet::Flow));
    }
}

fn handle_flow(
    current_flow: Res<State<GameFlow>>,
    mut next_flow: ResMut<NextState<GameFlow>>,
    mut flow: ResMut<Flow>,
    time: Res<Time>,
) {
    flow.tick(time.delta());
    if flow.timer.just_finished() {
        match current_flow.get() {
            GameFlow::RoundStart => next_flow.set(GameFlow::SelectElement),
            GameFlow::SelectElement => next_flow.set(GameFlow::SelectAction),
            GameFlow::SelectAction => next_flow.set(GameFlow::ResolveAction),
            GameFlow::ResolveAction => next_flow.set(GameFlow::ComboBreaker),
            GameFlow::ComboBreaker => next_flow.set(GameFlow::RoundOver),
            GameFlow::RoundOver => next_flow.set(GameFlow::SelectElement),
            _ => (),
        }
    }
}
