use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{Animator, Delay, TweenCompleted};

use crate::animations::{fade_in, fade_out, scale_down, scale_up};
use crate::config::{ANIM_FADE_IN, ANIM_SCALE_DOWN, ANIM_SCALE_UP, SIZE_XXXL, TRANSPARENT};
use crate::globals::AudioAssets;
use crate::helper::despawn;
use crate::schedule::GameSet;
use crate::state::UiState;
use crate::{globals::UiAssets, state::GameState};

const NEXT_STATE: u64 = 100;

pub struct RoundStartPlugin;

#[derive(Component, Debug)]
struct RoundStartPopup;

impl Plugin for RoundStartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameStart), on_enter.in_set(GameSet::Ui));
        app.add_systems(
            OnExit(GameState::GameStart),
            despawn::<RoundStartPopup>.in_set(GameSet::Ui),
        );
        app.add_systems(
            Update,
            on_complete
                .in_set(GameSet::Ui)
                .run_if(in_state(GameState::GameStart)),
        );
    }
}

fn on_enter(
    mut commands: Commands,
    mut ui_state: ResMut<NextState<UiState>>,
    ui_assets: Res<UiAssets>,
    audio_assets: Res<AudioAssets>,
) {
    ui_state.set(UiState::Title);
    let background_animation = fade_in().then(
        Delay::new(Duration::from_millis(
            (ANIM_SCALE_UP + ANIM_SCALE_DOWN) * 2 - ANIM_FADE_IN,
        ))
        .then(fade_out().with_completed_event(NEXT_STATE)),
    );
    let first_animation = scale_up().then(scale_down());
    let second_animation = Delay::new(Duration::from_millis(ANIM_SCALE_UP + ANIM_SCALE_DOWN))
        .then(scale_up().then(scale_down()));
    // Fading Screen
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            RoundStartPopup,
            BackgroundColor(TRANSPARENT),
            Animator::new(background_animation),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("SWIRLY"),
                TextFont {
                    font: ui_assets.ms_pain.clone(),
                    font_size: SIZE_XXXL,
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    ..default()
                },
                TextColor(Color::WHITE),
                Animator::new(first_animation),
            ));
            parent.spawn((
                Text::new("WHIRLY"),
                TextFont {
                    font: ui_assets.ms_pain.clone(),
                    font_size: SIZE_XXXL,
                    ..default()
                },
                Transform {
                    scale: Vec3::ZERO,
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    ..default()
                },
                TextColor(Color::WHITE),
                Animator::new(second_animation),
            ));
            parent.spawn(AudioPlayer::new(audio_assets.ready.clone()));
        });
}

fn on_complete(
    mut reader: EventReader<TweenCompleted>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for &event in reader.read() {
        if event.user_data == NEXT_STATE {
            game_state.set(GameState::SelectElement);
        }
    }
}
