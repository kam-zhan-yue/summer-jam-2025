use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{
    lens::{TransformScaleLens, UiPositionLens},
    Animator, Tween, TweenCompleted,
};

const RESOLVE_COMPLETE_ID: u64 = 1;
const BACK_TO_ELEMENT: u64 = 2;
const LOOP: u64 = 3;
const COMBO_BREAKER: u64 = 4;

use crate::{
    animations::{move_in_tween, move_out_tween},
    camera::{SCREEN_X, SCREEN_Y},
    combo::{GameData, ResolveResult},
    config::SIZE_M,
    events::ApplyEffectsEvent,
    globals::UiAssets,
    schedule::GameSet,
    state::GameState,
    types::{Outcome, Player},
};

#[derive(Component, Debug)]
struct TransitionTitle;

pub struct ResolveActionPlugin;

impl Plugin for ResolveActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::ResolveAction),
            on_enter.in_set(GameSet::Ui),
        );
        app.add_systems(
            Update,
            update_next_flow
                .in_set(GameSet::Ui)
                .run_if(in_state(GameState::ResolveAction)),
        );
    }
}

const IMAGE_WIDTH: f32 = 800.;
const IMAGE_HEIGHT: f32 = 400.;

fn on_enter(mut commands: Commands, ui_assets: Res<UiAssets>, game_data: Res<GameData>) {
    let result = game_data.get_action_result();
    let move_in_tween = move_in_tween(&IMAGE_WIDTH, &IMAGE_HEIGHT);

    let move_out_tween = move_out_tween(&IMAGE_WIDTH, &IMAGE_HEIGHT);

    let sequence = move_in_tween.then(move_out_tween.with_completed_event(RESOLVE_COMPLETE_ID));

    // Image
    commands.spawn((
        Node {
            width: Val::Px(800.0),
            height: Val::Px(400.0),
            left: Val::Px(-SCREEN_X),
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

fn update_next_flow(
    mut commands: Commands,
    mut reader: EventReader<TweenCompleted>,
    mut writer: EventWriter<ApplyEffectsEvent>,
    mut game_data: ResMut<GameData>,
    mut game_flow: ResMut<NextState<GameState>>,
    query: Query<Entity, With<TransitionTitle>>,
    ui_assets: Res<UiAssets>,
) {
    for event in reader.read() {
        match event.user_data {
            RESOLVE_COMPLETE_ID => {
                let result = game_data.get_action_result();
                game_data.process_turn();

                writer.send(ApplyEffectsEvent::default());
                if game_data.can_end_game() {
                    game_over(&mut game_flow);
                } else {
                    match (&result.outcome, game_data.action) {
                        (Outcome::Draw, 1) => back_to_element(&mut commands, &ui_assets),
                        (Outcome::Draw, _) => loop_action(
                            &mut commands,
                            &result,
                            &game_data,
                            &mut game_flow,
                            &ui_assets,
                        ),
                        (Outcome::PlayerOne, 1) => {
                            advantage(&mut commands, &result, &mut game_data, &ui_assets)
                        }
                        (Outcome::PlayerTwo, 1) => {
                            advantage(&mut commands, &result, &mut game_data, &ui_assets)
                        }
                        (Outcome::PlayerOne, _) => loop_action(
                            &mut commands,
                            &result,
                            &game_data,
                            &mut game_flow,
                            &ui_assets,
                        ),
                        (Outcome::PlayerTwo, _) => loop_action(
                            &mut commands,
                            &result,
                            &game_data,
                            &mut game_flow,
                            &ui_assets,
                        ),
                    }
                }
            }
            BACK_TO_ELEMENT => {
                for entity in &query {
                    commands.entity(entity).despawn_recursive();
                }
                game_data.action = 0;
                game_flow.set(GameState::SelectElement);
            }
            COMBO_BREAKER => {
                for entity in &query {
                    commands.entity(entity).despawn_recursive();
                }
                game_data.action = 0;
                game_flow.set(GameState::SelectElement);
            }
            LOOP => {
                for entity in &query {
                    commands.entity(entity).despawn_recursive();
                }
                game_flow.set(GameState::SelectAction);
            }
            _ => (),
        }
    }
}

fn back_to_element(commands: &mut Commands, ui_assets: &Res<UiAssets>) {
    let tween_scale = Tween::new(
        EaseFunction::BounceOut,
        Duration::from_millis(1500),
        TransformScaleLens {
            start: Vec3::splat(0.01),
            end: Vec3::ONE,
        },
    )
    .with_completed_event(BACK_TO_ELEMENT);
    // Show a title, then go back to the element stage
    commands
        .spawn((
            TransitionTitle,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_child((
            Text::new("No Advantage. Restarting Round."),
            TextFont {
                font_size: SIZE_M,
                font: ui_assets.ms_pain.clone(),
                ..default()
            },
            TextColor(Color::BLACK),
            Animator::new(tween_scale),
        ));
}

fn loop_action(
    commands: &mut Commands,
    result: &ResolveResult,
    game_data: &ResMut<GameData>,
    game_flow: &mut ResMut<NextState<GameState>>,
    ui_assets: &Res<UiAssets>,
) {
    match (&result.outcome, &game_data.advantage) {
        (Outcome::PlayerOne, Player::Two) => combo_breaker(commands, &ui_assets),
        (Outcome::PlayerTwo, Player::One) => combo_breaker(commands, &ui_assets),
        _ => game_flow.set(GameState::SelectAction),
    }
}

fn combo_breaker(commands: &mut Commands, ui_assets: &Res<UiAssets>) {
    let tween_scale = Tween::new(
        EaseFunction::BounceOut,
        Duration::from_millis(1500),
        TransformScaleLens {
            start: Vec3::splat(0.01),
            end: Vec3::ONE,
        },
    )
    .with_completed_event(COMBO_BREAKER);
    // Show a title, then go back to the element stage
    commands
        .spawn((
            TransitionTitle,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_child((
            Text::new("COMBO BREAKER!\nRestarting Round."),
            TextFont {
                font_size: SIZE_M,
                font: ui_assets.ms_pain.clone(),
                ..default()
            },
            TextColor(Color::BLACK),
            Animator::new(tween_scale),
        ));
}

fn advantage(
    commands: &mut Commands,
    result: &ResolveResult,
    game_data: &mut ResMut<GameData>,
    ui_assets: &Res<UiAssets>,
) {
    let mut text = String::from("NONE");
    match result.outcome {
        Outcome::PlayerOne => {
            text = "Player One has the Advantage!\nThe combat will continue until Player One loses"
                .to_string();
            game_data.advantage = Player::One
        }
        Outcome::PlayerTwo => {
            text = "Player Two has the Advantage!\nThe combat will continue until Player Two loses"
                .to_string();
            game_data.advantage = Player::Two
        }

        _ => (),
    }

    let tween_scale = Tween::new(
        EaseFunction::BounceOut,
        Duration::from_millis(1500),
        TransformScaleLens {
            start: Vec3::splat(0.01),
            end: Vec3::ONE,
        },
    )
    .with_completed_event(LOOP);
    // Show a title, then go back to the element stage
    commands
        .spawn((
            TransitionTitle,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_child((
            Text::new(text),
            TextFont {
                font_size: SIZE_M,
                font: ui_assets.ms_pain.clone(),
                ..default()
            },
            TextColor(Color::BLACK),
            Animator::new(tween_scale),
        ));
}

fn game_over(game_flow: &mut ResMut<NextState<GameState>>) {
    game_flow.set(GameState::GameOver);
}
