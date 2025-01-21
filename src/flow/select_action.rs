use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::UiBackgroundColorLens, Animator, Tween};

const LOSS_COLOUR: Color = Color::srgb(0.2, 0.2, 0.2);
const WON_COLOUR: Color = Color::srgb(0.2, 0.8, 0.2);

use crate::{
    combo::{GameData, PlayerData},
    globals::UiAssets,
    helper::{despawn, hide, show},
    schedule::GameSet,
    state::{GameFlow, UiFlow},
    types::{Outcome, Player},
};

use super::{countdown::Countdown, COUNTDOWN_TIME, REVEAL_TIME, TITLE_TIME};

#[derive(Component, Debug)]
struct SelectActionPopup;

#[derive(Component, Debug)]
struct SelectActionTitle;

#[derive(Component, Debug)]
struct RevealActionPopup;

#[derive(Component, Debug)]
struct PlayerOneAction;

#[derive(Component, Debug)]
struct PlayerTwoAction;

pub struct SelectActionPlugin;

impl Plugin for SelectActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);

        app.add_systems(
            OnEnter(GameFlow::SelectAction),
            on_enter.in_set(GameSet::Ui),
        );

        // Hide and Show SelectActionPopup on Title
        app.add_systems(
            OnEnter(UiFlow::Title),
            show::<SelectActionPopup>
                .in_set(GameSet::Ui)
                .run_if(in_state(GameFlow::SelectAction)),
        );

        app.add_systems(
            OnExit(UiFlow::Title),
            hide::<SelectActionPopup>
                .in_set(GameSet::Ui)
                .run_if(in_state(GameFlow::SelectAction)),
        );

        app.add_systems(
            Update,
            (handle_countdown, handle_input)
                .in_set(GameSet::Ui)
                .run_if(in_state(GameFlow::SelectAction)),
        );
        app.add_systems(
            OnExit(UiFlow::Title),
            despawn::<SelectActionTitle>
                .in_set(GameSet::Ui)
                .run_if(in_state(GameFlow::SelectAction)),
        );
        app.add_systems(
            OnEnter(UiFlow::Reveal),
            reveal
                .in_set(GameSet::Ui)
                .run_if(in_state(GameFlow::SelectAction)),
        );
        app.add_systems(
            OnExit(GameFlow::SelectAction),
            (despawn::<SelectActionPopup>, despawn::<RevealActionPopup>).in_set(GameSet::Ui),
        );
    }
}

fn setup(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            SelectActionPopup,
            Visibility::Hidden,
        ))
        .with_child((
            SelectActionTitle,
            Text::new("SELECT ACTION"),
            TextFont {
                font: ui_assets.fira_sans_bold.clone(),
                font_size: 72.0,
                ..default()
            },
            TextColor(Color::BLACK),
        ));
}

fn on_enter(mut countdown: ResMut<Countdown>, mut next_ui: ResMut<NextState<UiFlow>>) {
    countdown.reset(Timer::from_seconds(TITLE_TIME, TimerMode::Once));
    next_ui.set(UiFlow::Title);
}

fn handle_countdown(
    mut countdown: ResMut<Countdown>,
    current_ui_flow: Res<State<UiFlow>>,
    mut next_ui_flow: ResMut<NextState<UiFlow>>,
    mut next_game_flow: ResMut<NextState<GameFlow>>,
    time: Res<Time>,
) {
    countdown.tick(time.delta());
    if countdown.timer.just_finished() {
        match current_ui_flow.get() {
            // Go to the countdown after the title
            UiFlow::Title => {
                countdown.reset(Timer::from_seconds(COUNTDOWN_TIME, TimerMode::Once));
                next_ui_flow.set(UiFlow::Countdown)
            }
            // Go to the reveal after the countdown
            UiFlow::Countdown => {
                countdown.reset(Timer::from_seconds(REVEAL_TIME, TimerMode::Once));
                next_ui_flow.set(UiFlow::Reveal);
            }
            // Go to the next stage after the reveal
            UiFlow::Reveal => next_game_flow.set(GameFlow::ResolveAction),
        }
    }
}

fn handle_input(
    current_ui_flow: Res<State<UiFlow>>,
    input: Res<ButtonInput<KeyCode>>,
    mut game_data: ResMut<GameData>,
) {
    if *current_ui_flow.get() != UiFlow::Countdown {
        return;
    }
    process_input(&mut game_data.player_one, &input);
    process_input(&mut game_data.player_two, &input);
}

fn process_input(player_data: &mut PlayerData, input: &Res<ButtonInput<KeyCode>>) {
    // Get the selected choice
    let mut selected_choice = None;
    for (key, choice) in &player_data.input.map {
        if input.pressed(*key) {
            selected_choice = Some(choice);
            break;
        }
    }

    if let Some(choice) = selected_choice {
        if player_data.choice_selection.action != choice.action {
            player_data.choice_selection.action = choice.action;
        }
    }
}

fn won_tween() -> Tween<BackgroundColor> {
    Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_secs(1),
        UiBackgroundColorLens {
            start: Color::WHITE,
            end: WON_COLOUR,
        },
    )
}

fn loss_tween() -> Tween<BackgroundColor> {
    Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_secs(1),
        UiBackgroundColorLens {
            start: Color::WHITE,
            end: LOSS_COLOUR,
        },
    )
}

fn reveal(mut commands: Commands, game_data: Res<GameData>, ui_assets: Res<UiAssets>) {
    let result = game_data.get_action_result();
    let player_one_tween = match result.outcome {
        Outcome::PlayerOne => won_tween(),
        _ => loss_tween(),
    };

    let player_two_tween = match result.outcome {
        Outcome::PlayerTwo => won_tween(),
        _ => loss_tween(),
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                row_gap: Val::Px(50.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            RevealActionPopup,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    PlayerOneAction,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(200.0),
                        border: UiRect::all(Val::Px(10.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(Color::WHITE),
                    Animator::new(player_one_tween),
                ))
                .with_child((
                    ImageNode::new(ui_assets.get_icon(game_data.get_action(Player::One))),
                    Node {
                        width: Val::Px(75.0),
                        height: Val::Px(75.0),
                        ..default()
                    },
                ));
            parent
                .spawn((
                    PlayerTwoAction,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(200.0),
                        border: UiRect::all(Val::Px(10.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(Color::WHITE),
                    Animator::new(player_two_tween),
                ))
                .with_child((
                    ImageNode::new(ui_assets.get_icon(game_data.get_action(Player::Two))),
                    Node {
                        width: Val::Px(75.0),
                        height: Val::Px(75.0),
                        ..default()
                    },
                ));
        });
}
