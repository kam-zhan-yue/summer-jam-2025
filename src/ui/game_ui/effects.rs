use bevy::prelude::*;

use crate::combo::{GameData, MAX_HEALTH};
use crate::rhythm::Rhythm;
use crate::schedule::GameSet;
use crate::state::{GameFlow, GameState};

use super::ui_assets::UiAssets;

const EFFECTS_TIME: f32 = 2.0;

#[derive(Component, Debug)]
struct PlayerOneHealth;

#[derive(Component, Debug)]
struct PlayerTwoHealth;

#[derive(Component, Debug)]
struct HealthPopupItem;

#[derive(Component, Debug)]
struct HealthPopup;

#[derive(Component, Debug)]
struct EffectsPopup {
    timer: Timer,
}

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup);
        app.add_systems(OnEnter(GameFlow::ApplyEffects), on_enter_apply_effects);
        app.add_systems(
            Update,
            handle_timers
                .in_set(GameSet::Ui)
                .run_if(in_state(GameFlow::ApplyEffects)),
        );
    }
}

fn setup(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands.spawn((
        Name::new("EffectsPopup"),
        EffectsPopup {
            timer: Timer::from_seconds(EFFECTS_TIME, TimerMode::Once),
        },
    ));
    commands
        .spawn((
            Name::new("Health Popup"),
            HealthPopup,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            spawn_health(
                Name::new("Player One Health"),
                parent,
                PlayerOneHealth,
                JustifyContent::Start,
                FlexDirection::Row,
                &ui_assets,
            );
            spawn_health(
                Name::new("Player Two Health"),
                parent,
                PlayerTwoHealth,
                JustifyContent::End,
                FlexDirection::RowReverse,
                &ui_assets,
            )
        });
}

fn handle_timers(
    time: Res<Time>,
    mut query: Query<&mut EffectsPopup>,
    mut rhythm: ResMut<Rhythm>,
    mut game_state: ResMut<NextState<GameState>>,
    mut game_flow: ResMut<NextState<GameFlow>>,
    game_data: Res<GameData>,
) {
    let Ok(mut effects_popup) = query.get_single_mut() else {
        return;
    };

    effects_popup.timer.tick(time.delta());
    if effects_popup.timer.just_finished() {
        rhythm.reset();
        if game_data.can_end_game() {
            game_state.set(GameState::GameOver);
        } else {
            game_flow.set(GameFlow::Title);
        }
    }
}

fn spawn_health(
    name: Name,
    parent: &mut ChildBuilder,
    player: impl Component,
    justify_content: JustifyContent,
    flex_direction: FlexDirection,
    ui_assets: &Res<UiAssets>,
) {
    parent
        .spawn((
            name,
            player,
            Node {
                width: Val::Percent(50.0),
                justify_content,
                flex_direction,
                ..default()
            },
        ))
        .with_children(|parent| {
            for _ in 0..MAX_HEALTH {
                parent.spawn((
                    HealthPopupItem,
                    ImageNode::new(ui_assets.heart_full.clone()),
                    Node {
                        width: Val::Px(75.0),
                        height: Val::Px(75.0),
                        ..default()
                    },
                ));
            }
        });
}

fn on_enter_apply_effects(
    player_one_health_popup: Query<&Children, With<PlayerOneHealth>>,
    player_two_health_popup: Query<&Children, With<PlayerTwoHealth>>,
    mut health_popup_items: Query<&mut ImageNode, With<HealthPopupItem>>,
    game_data: Res<GameData>,
    ui_assets: Res<UiAssets>,
) {
    if let Ok(player_one_children) = player_one_health_popup.get_single() {
        apply_health_effects(
            player_one_children,
            game_data.player_one.health,
            &mut health_popup_items,
            &ui_assets,
        );
    }

    if let Ok(player_two_children) = player_two_health_popup.get_single() {
        apply_health_effects(
            player_two_children,
            game_data.player_two.health,
            &mut health_popup_items,
            &ui_assets,
        );
    }
}

fn apply_health_effects(
    children: &Children,
    health: i32,
    health_popup_items: &mut Query<&mut ImageNode, With<HealthPopupItem>>,
    ui_assets: &UiAssets,
) {
    for i in 0..MAX_HEALTH {
        if let Some(&child) = children.get(i as usize) {
            if let Ok(mut health_item) = health_popup_items.get_mut(child) {
                if health < i + 1 {
                    health_item.image = ui_assets.heart_broken.clone();
                } else {
                    health_item.image = ui_assets.heart_full.clone();
                }
            }
        }
    }
}
