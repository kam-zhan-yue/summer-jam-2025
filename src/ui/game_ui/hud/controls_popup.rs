use bevy::prelude::*;

use crate::{
    config::{CONTROLS_HEIGHT, CONTROLS_WIDTH},
    globals::UiAssets,
    helper::despawn,
    schedule::GameSet,
    settings::{GameMode, GameSettings},
    state::{GameState, UiState},
};

const CONTROLS_SIZE_MULTIPLIER: f32 = 0.75;

#[derive(Component, Debug)]
struct ControlsPopup;

pub struct ControlsPopupPlugin;

impl Plugin for ControlsPopupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::Countdown), setup.in_set(GameSet::Ui));
        app.add_systems(OnExit(UiState::Countdown), despawn::<ControlsPopup>);
    }
}

fn setup(
    mut commands: Commands,
    settings: Res<GameSettings>,
    current_state: Res<State<GameState>>,
    ui_assets: Res<UiAssets>,
) {
    let state = current_state.get();
    let (chart, p1_controls, p2_controls) = match state {
        GameState::SelectElement => (
            ui_assets.chart_elements.clone(),
            ui_assets.controls_elements_p1.clone(),
            ui_assets.controls_elements_p2.clone(),
        ),
        GameState::SelectAction => (
            ui_assets.chart_actions.clone(),
            ui_assets.controls_actions_p1.clone(),
            ui_assets.controls_actions_p2.clone(),
        ),
        _ => (
            ui_assets.chart_combos.clone(),
            ui_assets.controls_elements_p1.clone(),
            ui_assets.controls_elements_p2.clone(),
        ),
    };

    commands
        .spawn((
            ControlsPopup,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            // Root Node for Chart
            parent
                .spawn((Node {
                    width: Val::Auto,
                    height: Val::Auto,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },))
                .with_child((
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(200.0),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    ImageNode::new(chart),
                ));
            // Nodes for the P1 and P2 Controls
            parent.spawn((
                Node {
                    width: Val::Px(CONTROLS_WIDTH * CONTROLS_SIZE_MULTIPLIER),
                    height: Val::Px(CONTROLS_HEIGHT * CONTROLS_SIZE_MULTIPLIER),
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(20.0),
                    left: Val::Px(75.0),
                    ..default()
                },
                ImageNode::new(p1_controls),
            ));
            if settings.game_mode == GameMode::TwoPlayer {
                parent.spawn((
                    Node {
                        width: Val::Px(CONTROLS_WIDTH * CONTROLS_SIZE_MULTIPLIER),
                        height: Val::Px(CONTROLS_HEIGHT * CONTROLS_SIZE_MULTIPLIER),
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(20.0),
                        right: Val::Px(75.0),
                        ..default()
                    },
                    ImageNode::new(p2_controls),
                ));
            }
        });
}
