use bevy::prelude::*;

use crate::{
    combo::GameData,
    config::{BUTTON_BORDER, BUTTON_HEIGHT, BUTTON_WIDTH, NORMAL_BUTTON, SIZE_M, SIZE_XL},
    globals::{AudioAssets, UiAssets},
    helper::{despawn, handle_buttons},
    schedule::GameSet,
    state::GameState,
    types::Player,
};

#[derive(Component, Debug)]
struct RoundOverPopup;

#[derive(Component, Debug)]
pub struct TitleButton;

pub struct RoundOverPlugin;

impl Plugin for RoundOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), on_enter);
        app.add_systems(
            Update,
            (handle_buttons, handle_title_button)
                .in_set(GameSet::Ui)
                .run_if(in_state(GameState::GameOver)),
        );
        app.add_systems(OnExit(GameState::GameOver), despawn::<RoundOverPopup>);
    }
}

fn on_enter(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    audio_assets: Res<AudioAssets>,
    game_data: Res<GameData>,
) {
    let winner = game_data.get_winner();
    let audio = match winner {
        Player::One => audio_assets.player_one_wins.clone(),
        Player::Two => audio_assets.player_two_wins.clone(),
    };

    let text = match winner {
        Player::One => "Red Wins!",
        Player::Two => "Blue Wins!",
    };

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
            RoundOverPopup,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(AudioPlayer::new(audio)),
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    ..default()
                })
                .with_children(|parent| {
                    // Text
                    parent.spawn((
                        Text::new(text),
                        TextFont {
                            font: ui_assets.ms_pain.clone(),
                            font_size: SIZE_XL,
                            ..default()
                        },
                        TextColor::BLACK,
                        Label,
                    ));
                });
            // Title Player Button
            parent
                .spawn((
                    TitleButton,
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
                    BorderRadius::all(Val::Px(5.0)),
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_child((
                    Text::new("Back to Title"),
                    TextFont {
                        font: ui_assets.ms_pain.clone(),
                        font_size: SIZE_M,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });
}

fn handle_title_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<TitleButton>)>,
    mut game_flow: ResMut<NextState<GameState>>,
) {
    let Ok(interaction) = interaction_query.get_single() else {
        return;
    };

    if *interaction == Interaction::Pressed {
        game_flow.set(GameState::Title);
    }
}
