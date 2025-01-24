use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{Animator, Delay, TweenCompleted};

use crate::{
    animations::{fade_in, fade_out, scale_down, scale_up},
    combo::{GameData, PlayerData},
    config::{
        ANIM_FADE_IN, ANIM_SCALE_DOWN, ANIM_SCALE_UP, COUNTDOWN_TIME, REVEAL_TIME, SIZE_XXL,
        TRANSPARENT,
    },
    events::SelectElementEvent,
    globals::{AudioAssets, UiAssets},
    helper::{despawn, get_random},
    schedule::GameSet,
    state::{GameState, UiState},
    types::{Choice, Element, Player},
};

use super::countdown::Countdown;

pub const COUNTDOWN_STATE: u64 = 200;

#[derive(Component, Debug)]
struct SelectElementPopup;

#[derive(Component, Debug)]
struct SelectElementTitle;

pub struct SelectElementPlugin;

impl Plugin for SelectElementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::SelectElement),
            on_enter.in_set(GameSet::Ui),
        );
        app.add_systems(
            Update,
            (handle_countdown, handle_input, select_audio)
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

        // Depsawn after exiting SelectElement
        app.add_systems(
            OnExit(GameState::SelectElement),
            despawn::<SelectElementPopup>.in_set(GameSet::Ui),
        );
    }
}

fn on_enter(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    mut next_ui: ResMut<NextState<UiState>>,
    audio_assets: Res<AudioAssets>,
) {
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
            Node {
                position_type: PositionType::Absolute,
                ..default()
            },
            TextColor(Color::WHITE),
            Animator::new(title_animation),
            AudioPlayer::new(audio_assets.select_element.clone()),
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
            next_ui_flow.set(UiState::Countdown);
        }
    }

    if countdown.timer.just_finished() {
        match current_ui_flow.get() {
            // Go to the reveal after the countdown
            UiState::Countdown => {
                next_ui_flow.set(UiState::None);
                countdown.reset(Timer::from_seconds(REVEAL_TIME, TimerMode::Once));
                next_game_flow.set(GameState::SelectAction);
            }
            _ => (),
        }
    }
}

fn handle_input(
    current_ui_flow: Res<State<UiState>>,
    input: Res<ButtonInput<KeyCode>>,
    mut game_data: ResMut<GameData>,
    mut writer: EventWriter<SelectElementEvent>,
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
    writer: &mut EventWriter<SelectElementEvent>,
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
        player_data.select_element(player, choice.element, writer);
    }
}

fn select_audio(
    mut commands: Commands,
    popup_query: Query<Entity, With<SelectElementPopup>>,
    mut reader: EventReader<SelectElementEvent>,
    audio_assets: Res<AudioAssets>,
) {
    for event in reader.read() {
        let audio = match event.element {
            Choice::Element(Element::Fire) => audio_assets.select_fire.clone(),
            Choice::Element(Element::Water) => audio_assets.select_water.clone(),
            Choice::Element(Element::Grass) => audio_assets.select_grass.clone(),
            _ => get_random(&audio_assets.select_generic).clone(),
        };
        if let Ok(popup) = popup_query.get_single() {
            commands.entity(popup).insert(AudioPlayer::new(audio));
        }
    }
}
