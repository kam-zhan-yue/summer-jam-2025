use crate::config::{
    BORDER_RADIUS, BUTTON_BORDER, BUTTON_HEIGHT, BUTTON_WIDTH, NORMAL_BUTTON, SIZE_M, START_STATE,
};
use crate::globals::UiAssets;
use crate::helper::{despawn, handle_buttons};
use crate::schedule::GameSet;
use crate::settings::{GameMode, GameSettings};
use crate::state::GameState;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct MainMenu;

#[derive(Component, Debug)]
pub struct SinglePlayerButton;

#[derive(Component, Debug)]
pub struct TwoPlayerButton;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Title),
            spawn_main_menu.in_set(GameSet::Ui),
        );
        app.add_systems(
            Update,
            (
                handle_buttons,
                handle_single_player_button,
                handle_two_player_button,
            )
                .in_set(GameSet::Ui)
                .run_if(in_state(GameState::Title)),
        );
        app.add_systems(OnExit(GameState::Title), despawn::<MainMenu>);
    }
}

pub fn spawn_main_menu(mut commands: Commands, ui_assets: Res<UiAssets>) {
    // Spawn the Root Node
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                row_gap: Val::Px(8.),
                column_gap: Val::Px(8.),
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            // Title
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    ..default()
                })
                .with_children(|parent| {
                    // Logo
                    parent.spawn((
                        ImageNode::new(ui_assets.logo.clone()),
                        Node {
                            width: Val::Px(512.0),
                            height: Val::Px(256.0),
                            ..default()
                        },
                    ));
                });
            // Single Player Button
            parent
                .spawn((
                    SinglePlayerButton,
                    Button,
                    Node {
                        width: BUTTON_WIDTH,
                        height: BUTTON_HEIGHT,
                        border: BUTTON_BORDER,
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::all(BORDER_RADIUS),
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_child((
                    Text::new("1 Player"),
                    TextFont {
                        font: ui_assets.ms_pain.clone(),
                        font_size: SIZE_M,
                        ..default()
                    },
                    TextColor::WHITE,
                ));
            // Two Player Button
            parent
                .spawn((
                    TwoPlayerButton,
                    Button,
                    Node {
                        width: BUTTON_WIDTH,
                        height: BUTTON_HEIGHT,
                        border: BUTTON_BORDER,
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::all(BORDER_RADIUS),
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_child((
                    Text::new("2 Players"),
                    TextFont {
                        font: ui_assets.ms_pain.clone(),
                        font_size: SIZE_M,
                        ..default()
                    },
                    TextColor::WHITE,
                ));
        });
}

fn handle_single_player_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<SinglePlayerButton>)>,
    mut game_flow: ResMut<NextState<GameState>>,
    mut settings: ResMut<GameSettings>,
) {
    let Ok(interaction) = interaction_query.get_single() else {
        return;
    };

    if *interaction == Interaction::Pressed {
        settings.game_mode = GameMode::SinglePlayer;
        game_flow.set(START_STATE)
    }
}

fn handle_two_player_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<TwoPlayerButton>)>,
    mut game_flow: ResMut<NextState<GameState>>,
    mut settings: ResMut<GameSettings>,
) {
    let Ok(interaction) = interaction_query.get_single() else {
        return;
    };

    if *interaction == Interaction::Pressed {
        settings.game_mode = GameMode::TwoPlayer;
        game_flow.set(START_STATE);
    }
}
