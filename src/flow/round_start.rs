use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{Animator, Delay};

use crate::animations::{fade_in, fade_out, scale_down, scale_up};
use crate::config::{ANIM_SCALE_DOWN, ANIM_SCALE_UP, ROUND_START_TIME, SIZE_XXL, TRANSPARENT};
use crate::helper::despawn;
use crate::schedule::GameSet;
use crate::{globals::UiAssets, state::GameState};

const TITLE_ANIMATION: u64 = 100;

use super::Flow;

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
    }
}

fn on_enter(mut commands: Commands, ui_assets: Res<UiAssets>, mut flow: ResMut<Flow>) {
    flow.reset(Timer::from_seconds(ROUND_START_TIME, TimerMode::Once));

    let background_animation = fade_in()
        .then(Delay::new(Duration::from_millis(ANIM_SCALE_UP + ANIM_SCALE_DOWN)).then(fade_out()));
    let first_animation = scale_up().then(scale_down().with_completed_event(TITLE_ANIMATION));
    let second_animation = Delay::new(Duration::from_millis(ANIM_SCALE_UP))
        .then(scale_up().then(scale_down().with_completed_event(TITLE_ANIMATION)));
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
                Text::new("GAME"),
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
                Animator::new(first_animation),
            ));
            parent.spawn((
                Text::new("START"),
                TextFont {
                    font: ui_assets.ms_pain.clone(),
                    font_size: SIZE_XXL,
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
        });
}
