use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{Animator, Delay, TweenCompleted};

pub const COUNTDOWN_STATE: u64 = 800;

use crate::{
    animations::{fade_in, fade_out, scale_down, scale_up},
    combo::{GameData, PlayerData},
    config::{ANIM_FADE_IN, ANIM_SCALE_DOWN, ANIM_SCALE_UP, COUNTDOWN_TIME, SIZE_XXL, TRANSPARENT},
    events::SelectActionEvent,
    globals::UiAssets,
    helper::{despawn, hide, show},
    schedule::GameSet,
    state::{GameState, UiState},
    types::Player,
};

use super::countdown::Countdown;

#[derive(Component, Debug)]
struct SelectActionPopup;

#[derive(Component, Debug)]
struct SelectActionTitle;

#[derive(Component, Debug)]
struct RevealActionPopup;

pub struct SelectActionPlugin;

impl Plugin for SelectActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::SelectAction),
            on_enter.in_set(GameSet::Ui),
        );

        // Hide and Show SelectActionPopup on Title
        app.add_systems(
            OnEnter(UiState::Title),
            show::<SelectActionPopup>
                .in_set(GameSet::Ui)
                .run_if(in_state(GameState::SelectAction)),
        );

        app.add_systems(
            OnExit(UiState::Title),
            hide::<SelectActionPopup>
                .in_set(GameSet::Ui)
                .run_if(in_state(GameState::SelectAction)),
        );

        app.add_systems(
            Update,
            (handle_countdown, handle_input)
                .in_set(GameSet::Ui)
                .run_if(in_state(GameState::SelectAction)),
        );
        app.add_systems(
            OnExit(UiState::Title),
            despawn::<SelectActionTitle>
                .in_set(GameSet::Ui)
                .run_if(in_state(GameState::SelectAction)),
        );
        app.add_systems(
            OnExit(GameState::SelectAction),
            (despawn::<SelectActionPopup>, despawn::<RevealActionPopup>).in_set(GameSet::Ui),
        );
    }
}

fn on_enter(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    mut countdown: ResMut<Countdown>,
    mut next_ui: ResMut<NextState<UiState>>,
    game_data: Res<GameData>,
) {
    if game_data.action >= 1 {
        countdown.reset(Timer::from_seconds(COUNTDOWN_TIME, TimerMode::Once));
        next_ui.set(UiState::Countdown);
        return;
    }

    next_ui.set(UiState::Title);

    let background_animation = fade_in().then(
        Delay::new(Duration::from_millis(
            ANIM_SCALE_UP + ANIM_SCALE_DOWN - ANIM_FADE_IN,
        ))
        .then(fade_out().with_completed_event(COUNTDOWN_STATE)),
    );
    let title_animation = scale_up().then(scale_down());
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(TRANSPARENT),
            Animator::new(background_animation),
            SelectActionPopup,
        ))
        .with_child((
            SelectActionTitle,
            Text::new("FIGHT!"),
            TextFont {
                font: ui_assets.ms_pain.clone(),
                font_size: SIZE_XXL,
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                ..default()
            },
            TextColor(Color::WHITE),
            Animator::new(title_animation),
        ));
}

fn handle_countdown(
    mut countdown: ResMut<Countdown>,
    current_ui_flow: Res<State<UiState>>,
    mut next_ui_flow: ResMut<NextState<UiState>>,
    mut next_game_flow: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut reader: EventReader<TweenCompleted>,
) {
    countdown.tick(time.delta());
    for event in reader.read() {
        if event.user_data == COUNTDOWN_STATE {
            countdown.reset(Timer::from_seconds(COUNTDOWN_TIME, TimerMode::Once));
            next_ui_flow.set(UiState::Countdown)
        }
    }

    if countdown.timer.just_finished() {
        match current_ui_flow.get() {
            // Go to the reveal after the countdown
            UiState::Countdown => next_game_flow.set(GameState::ResolveAction),
            _ => (),
        }
    }
}

fn handle_input(
    current_ui_flow: Res<State<UiState>>,
    input: Res<ButtonInput<KeyCode>>,
    mut game_data: ResMut<GameData>,
    mut writer: EventWriter<SelectActionEvent>,
) {
    if *current_ui_flow.get() != UiState::Countdown {
        return;
    }
    process_input(&mut game_data.player_one, &input, Player::One, &mut writer);
    process_input(&mut game_data.player_two, &input, Player::Two, &mut writer);
}

fn process_input(
    player_data: &mut PlayerData,
    input: &Res<ButtonInput<KeyCode>>,
    player: Player,
    writer: &mut EventWriter<SelectActionEvent>,
) {
    // Get the selected choice
    let mut selected_choice = None;
    for (key, choice) in &player_data.input.map {
        if input.pressed(*key) {
            selected_choice = Some(choice);
            break;
        }
    }

    if let Some(choice) = selected_choice {
        player_data.select_action(player, choice.action, writer);
    }
}
