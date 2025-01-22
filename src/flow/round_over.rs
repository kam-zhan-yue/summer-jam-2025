use bevy::prelude::*;

use crate::{
    globals::UiAssets,
    helper::{despawn, handle_buttons, NORMAL_BUTTON},
    schedule::GameSet,
    state::GameFlow,
};

#[derive(Component, Debug)]
struct RoundOverPopup;

#[derive(Component, Debug)]
pub struct TitleButton;

pub struct RoundOverPlugin;

impl Plugin for RoundOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameFlow::RoundOver), on_enter);
        app.add_systems(
            Update,
            (handle_buttons, handle_title_button)
                .in_set(GameSet::Ui)
                .run_if(in_state(GameFlow::RoundOver)),
        );
        app.add_systems(OnExit(GameFlow::RoundOver), despawn::<RoundOverPopup>);
    }
}

fn on_enter(mut commands: Commands, ui_assets: Res<UiAssets>) {
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
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    ..default()
                })
                .with_children(|parent| {
                    // Text
                    parent.spawn((
                        Text::new("Game Over!"),
                        TextFont {
                            font: ui_assets.fira_sans_bold.clone(),
                            font_size: 50.0,
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
                        width: Val::Px(150.0),
                        height: Val::Px(40.0),
                        border: UiRect::all(Val::Px(1.0)),
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
                        font: ui_assets.fira_sans_bold.clone(),
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });
}

fn handle_title_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<TitleButton>)>,
    mut game_flow: ResMut<NextState<GameFlow>>,
) {
    let Ok(interaction) = interaction_query.get_single() else {
        return;
    };

    if *interaction == Interaction::Pressed {
        game_flow.set(GameFlow::Title);
    }
}
