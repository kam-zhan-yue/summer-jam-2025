use crate::helper::{despawn, handle_buttons, NORMAL_BUTTON};
use crate::schedule::GameSet;
use crate::state::GameFlow;
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
        app.add_systems(OnEnter(GameState::Start), spawn_main_menu);
        app.add_systems(
            Update,
            (handle_buttons, handle_single_player_button)
                .in_set(GameSet::Ui)
                .run_if(in_state(GameState::Start)),
        );
        app.add_systems(OnExit(GameState::Start), despawn::<MainMenu>);
    }
}

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                    // Text
                    parent.spawn((
                        Text::new("Swirlie, Wedgie, Willie!"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 50.0,
                            ..default()
                        },
                        Label,
                    ));
                });
            // Single Player Button
            parent
                .spawn((
                    SinglePlayerButton,
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
                    Text::new("1 Player"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
            // Two Player Button
            parent
                .spawn((
                    TwoPlayerButton,
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
                    Text::new("2 Players"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });
}

fn handle_single_player_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<SinglePlayerButton>)>,
    mut game_state: ResMut<NextState<GameState>>,
    mut game_flow: ResMut<NextState<GameFlow>>,
) {
    let Ok(interaction) = interaction_query.get_single() else {
        return;
    };

    if *interaction == Interaction::Pressed {
        game_state.set(GameState::Game);
        game_flow.set(GameFlow::Title);
    }
}
