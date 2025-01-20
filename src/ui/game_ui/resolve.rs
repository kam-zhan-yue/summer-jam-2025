use crate::combo::{GameData, ResolveEvent};
use crate::helper::despawn;
use crate::rhythm::Rhythm;
use crate::schedule::GameSet;
use crate::state::GameFlow;
use bevy::prelude::*;

const RESOLVE_TIME: f32 = 2.;

#[derive(Component, Debug)]
struct Resolve {
    timer: Timer,
}

#[derive(Component, Debug)]
struct PlayerOneChoice;

#[derive(Component, Debug)]
struct PlayerTwoChoice;

pub struct ResolvePlugin;

impl Plugin for ResolvePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameFlow::Reveal), setup.in_set(GameSet::Ui));
        app.add_systems(
            Update,
            handle_resolve
                .in_set(GameSet::Ui)
                .run_if(in_state(GameFlow::Reveal)),
        );
        app.add_systems(OnExit(GameFlow::Title), despawn::<Resolve>);
    }
}

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    // Root Node
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                row_gap: Val::Px(50.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            Resolve {
                timer: Timer::from_seconds(RESOLVE_TIME, TimerMode::Once),
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    PlayerOneChoice,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(200.0),
                        border: UiRect::all(Val::Px(10.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(Color::WHITE),
                ))
                .with_child((
                    Text::new("Player One"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::BLACK),
                ));
            parent
                .spawn((
                    PlayerTwoChoice,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(200.0),
                        border: UiRect::all(Val::Px(10.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(Color::WHITE),
                ))
                .with_child((
                    Text::new("Player Two"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::BLACK),
                ));
        });
}

fn handle_resolve(
    rhythm: Res<Rhythm>,
    mut player_one_choice: Query<
        (&mut BackgroundColor, &mut BorderColor, &Children),
        With<PlayerOneChoice>,
    >,
    mut player_two_choice: Query<
        (&mut BackgroundColor, &mut BorderColor, &Children),
        (With<PlayerTwoChoice>, Without<PlayerOneChoice>),
    >,
    mut text_query: Query<&mut Text>,
    game_data: Res<GameData>,
) {
    let Ok(player_one) = player_one_choice.get_single_mut() else {
        return;
    };

    let Ok(player_two) = player_two_choice.get_single_mut() else {
        return;
    };

    if let Ok(mut text) = text_query.get_mut(player_one.2[0]) {
        **text = game_data
            .player_one
            .choice_selection
            .get_choice(rhythm.beat)
            .to_string();
    }
    if let Ok(mut text) = text_query.get_mut(player_two.2[0]) {
        **text = game_data
            .player_two
            .choice_selection
            .get_choice(rhythm.beat)
            .to_string();
    }
}
