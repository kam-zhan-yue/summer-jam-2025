use bevy::prelude::*;
use std::time::Duration;

mod countdown;
mod resolve_action;
mod round_over;
mod round_start;
mod select_action;
mod select_element;

use countdown::CountdownPlugin;
use resolve_action::ResolveActionPlugin;
use round_over::RoundOverPlugin;
use round_start::RoundStartPlugin;
use select_action::SelectActionPlugin;
use select_element::SelectElementPlugin;

use crate::{schedule::GameSet, state::GameState};

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
        app.add_plugins(RoundOverPlugin);
        app.add_plugins(CountdownPlugin);
        app.add_systems(Update, handle_flow.in_set(GameSet::Flow));
    }
}

fn handle_flow(
    current_flow: Res<State<GameState>>,
    mut next_flow: ResMut<NextState<GameState>>,
    mut flow: ResMut<Flow>,
    time: Res<Time>,
) {
    flow.tick(time.delta());
    if flow.timer.just_finished() {
        match current_flow.get() {
            GameState::GameStart => next_flow.set(GameState::SelectElement),
            GameState::SelectElement => next_flow.set(GameState::SelectAction),
            GameState::SelectAction => next_flow.set(GameState::ResolveAction),
            GameState::ResolveAction => next_flow.set(GameState::GameOver),
            GameState::GameOver => next_flow.set(GameState::SelectElement),
            _ => (),
        }
    }
}
