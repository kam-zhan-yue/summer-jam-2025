use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::UiPositionLens, Animator, Tween};

use crate::{
    camera::{SCREEN_X, SCREEN_Y},
    combo::GameData,
    globals::UiAssets,
    schedule::GameSet,
    state::GameFlow,
};

pub struct ResolveActionPlugin;

impl Plugin for ResolveActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameFlow::ResolveAction),
            on_enter.in_set(GameSet::Ui),
        );
    }
}

const IMAGE_WIDTH: f32 = 800.;
const IMAGE_HEIGHT: f32 = 400.;
const OFFSET: f32 = 500.;

fn on_enter(mut commands: Commands, ui_assets: Res<UiAssets>, game_data: Res<GameData>) {
    let result = game_data.get_action_result();
    let move_in_tween = Tween::new(
        EaseFunction::QuarticOut,
        Duration::from_millis(1200),
        UiPositionLens {
            start: UiRect {
                left: Val::Px(-SCREEN_X - OFFSET - IMAGE_WIDTH),
                top: Val::Px(SCREEN_Y / 2. - IMAGE_HEIGHT / 2.),
                right: Val::Auto,
                bottom: Val::Auto,
            },
            end: UiRect {
                left: Val::Px(SCREEN_X / 2. - IMAGE_WIDTH / 2.),
                top: Val::Px(SCREEN_Y / 2. - IMAGE_HEIGHT / 2.),
                right: Val::Auto,
                bottom: Val::Auto,
            },
        },
    );

    let move_out_tween = Tween::new(
        EaseFunction::QuarticIn,
        Duration::from_millis(1200),
        UiPositionLens {
            start: UiRect {
                left: Val::Px(SCREEN_X / 2. - IMAGE_WIDTH / 2.),
                top: Val::Px(SCREEN_Y / 2. - IMAGE_HEIGHT / 2.),
                right: Val::Auto,
                bottom: Val::Auto,
            },
            end: UiRect {
                left: Val::Px(SCREEN_X + OFFSET + IMAGE_WIDTH),
                top: Val::Px(SCREEN_Y / 2. - IMAGE_HEIGHT / 2.),
                right: Val::Auto,
                bottom: Val::Auto,
            },
        },
    );

    let sequence = move_in_tween.then(move_out_tween);

    commands.spawn((
        Node {
            width: Val::Px(800.0),
            height: Val::Px(400.0),
            left: Val::Px(-SCREEN_X - OFFSET),
            top: Val::Px(SCREEN_Y / 2.),
            right: Val::Auto,
            bottom: Val::Auto,
            border: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        ImageNode::new(ui_assets.get_result(result)),
        Animator::new(sequence),
        BorderColor(Color::BLACK),
        BorderRadius::ZERO,
    ));
}
