use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    combo::GameData,
    config::{BOT_TIME_MAX, BOT_TIME_MIN},
    events::{SelectActionEvent, SelectElementEvent},
    schedule::GameSet,
    settings::{GameMode, GameSettings},
    state::{GameState, UiState},
    types::{Action, Choice, Element, Player},
};

#[derive(Component, Debug)]
pub struct ComputerPlayer {
    timer: Timer,
}

impl ComputerPlayer {
    pub fn reset(&mut self) {
        self.timer = Timer::from_seconds(self.get_random_duration(), TimerMode::Repeating);
    }

    fn get_random_duration(&self) -> f32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(BOT_TIME_MIN..BOT_TIME_MAX)
    }
}

pub struct ComputerPlugin;

impl Plugin for ComputerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameStart), setup.in_set(GameSet::Flow));
        app.add_systems(
            Update,
            update_bot_element
                .run_if(in_state(UiState::Countdown))
                .run_if(in_state(GameState::SelectElement))
                .in_set(GameSet::Ui),
        );
        app.add_systems(
            Update,
            update_bot_action
                .run_if(in_state(UiState::Countdown))
                .run_if(in_state(GameState::SelectAction))
                .in_set(GameSet::Ui),
        );
    }
}

fn setup(mut commands: Commands, settings: Res<GameSettings>) {
    if settings.game_mode == GameMode::SinglePlayer {
        commands.spawn(ComputerPlayer {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        });
    }
}

fn update_bot_element(
    mut query: Query<&mut ComputerPlayer>,
    mut game_data: ResMut<GameData>,
    mut writer: EventWriter<SelectElementEvent>,
    settings: Res<GameSettings>,
    time: Res<Time>,
) {
    if settings.game_mode == GameMode::TwoPlayer {
        return;
    }
    let Ok(mut computer) = query.get_single_mut() else {
        return;
    };
    computer.timer.tick(time.delta());
    if computer.timer.just_finished() {
        let random_element = Choice::Element(Element::random());
        game_data
            .player_two
            .select_element(Player::Two, random_element, &mut writer);
        computer.reset();
    }
}

fn update_bot_action(
    mut query: Query<&mut ComputerPlayer>,
    mut game_data: ResMut<GameData>,
    mut writer: EventWriter<SelectActionEvent>,
    settings: Res<GameSettings>,
    time: Res<Time>,
) {
    if settings.game_mode == GameMode::TwoPlayer {
        return;
    }
    let Ok(mut computer) = query.get_single_mut() else {
        return;
    };
    computer.timer.tick(time.delta());
    if computer.timer.just_finished() {
        let element = game_data.player_two.choice_selection.element;
        let random_action = Choice::Action(Action::weighted(&element));
        game_data
            .player_two
            .select_action(Player::Two, random_action, &mut writer);
        computer.reset();
    }
}
