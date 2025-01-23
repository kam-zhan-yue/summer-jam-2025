use bevy::prelude::*;

use crate::{
    combo::{GameData, PlayerData},
    config::{COUNTDOWN_TIME, REVEAL_TIME, SIZE_XXL, TITLE_TIME},
    globals::UiAssets,
    helper::despawn,
    schedule::GameSet,
    state::{GameState, UiState},
    types::Player,
};

use super::countdown::Countdown;

#[derive(Component, Debug)]
struct SelectElementPopup;

#[derive(Component, Debug)]
struct SelectElementTitle;

#[derive(Component, Debug)]
struct RevealElementPopup;
#[derive(Component, Debug)]
struct PlayerOneElement;

#[derive(Component, Debug)]
struct PlayerTwoElement;

pub struct SelectElementPlugin;

impl Plugin for SelectElementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::SelectElement),
            on_enter.in_set(GameSet::Ui),
        );
        app.add_systems(
            Update,
            (handle_countdown, handle_input)
                .in_set(GameSet::Ui)
                .run_if(in_state(GameState::SelectElement)),
        );
        // Hide the title when out of the Title Flow
        app.add_systems(
            OnExit(UiState::Title),
            despawn::<SelectElementTitle>
                .in_set(GameSet::Ui)
                .run_if(in_state(GameState::SelectElement)),
        );

        app.add_systems(
            OnEnter(UiState::Reveal),
            reveal
                .in_set(GameSet::Ui)
                .run_if(in_state(GameState::SelectElement)),
        );

        // Depsawn after exiting SelectElement
        app.add_systems(
            OnExit(GameState::SelectElement),
            (despawn::<RevealElementPopup>, despawn::<SelectElementPopup>).in_set(GameSet::Ui),
        );
    }
}

fn on_enter(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    mut countdown: ResMut<Countdown>,
    mut next_ui: ResMut<NextState<UiState>>,
) {
    countdown.reset(Timer::from_seconds(TITLE_TIME, TimerMode::Once));
    next_ui.set(UiState::Title);
    println!("Entering Select Element");
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            SelectElementPopup,
        ))
        .with_child((
            SelectElementTitle,
            Text::new("SELECT ELEMENT"),
            TextFont {
                font: ui_assets.ms_pain.clone(),
                font_size: SIZE_XXL,
                ..default()
            },
            TextColor(Color::BLACK),
        ));
}

fn handle_countdown(
    mut countdown: ResMut<Countdown>,
    current_ui_flow: Res<State<UiState>>,
    mut next_ui_flow: ResMut<NextState<UiState>>,
    mut next_game_flow: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    countdown.tick(time.delta());
    if countdown.timer.just_finished() {
        match current_ui_flow.get() {
            // Go to the countdown after the title
            UiState::Title => {
                countdown.reset(Timer::from_seconds(COUNTDOWN_TIME, TimerMode::Once));
                next_ui_flow.set(UiState::Countdown)
            }
            // Go to the reveal after the countdown
            UiState::Countdown => {
                countdown.reset(Timer::from_seconds(REVEAL_TIME, TimerMode::Once));
                next_ui_flow.set(UiState::Reveal);
            }
            // Go to the next stage after the reveal
            UiState::Reveal => next_game_flow.set(GameState::SelectAction),
        }
    }
}

fn handle_input(
    current_ui_flow: Res<State<UiState>>,
    input: Res<ButtonInput<KeyCode>>,
    mut game_data: ResMut<GameData>,
) {
    if *current_ui_flow.get() != UiState::Countdown {
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
        if player_data.choice_selection.element != choice.element {
            player_data.choice_selection.element = choice.element;
        }
    }
}

fn reveal(mut commands: Commands, game_data: Res<GameData>, ui_assets: Res<UiAssets>) {
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
            RevealElementPopup,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    PlayerOneElement,
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
                ))
                .with_child((
                    ImageNode::new(ui_assets.get_icon(game_data.get_element(Player::One))),
                    Node {
                        width: Val::Px(75.0),
                        height: Val::Px(75.0),
                        ..default()
                    },
                ));
            parent
                .spawn((
                    PlayerTwoElement,
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
                ))
                .with_child((
                    ImageNode::new(ui_assets.get_icon(game_data.get_element(Player::Two))),
                    Node {
                        width: Val::Px(75.0),
                        height: Val::Px(75.0),
                        ..default()
                    },
                ));
        });
}
