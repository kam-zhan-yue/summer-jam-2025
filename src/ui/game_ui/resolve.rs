use crate::combo::GameData;
use crate::helper::despawn;
use crate::rhythm::Rhythm;
use crate::schedule::GameSet;
use crate::state::GameFlow;
use crate::types::Outcome;
use bevy::prelude::*;

const LOSE_COLOUR: Color = Color::srgb(0.3, 0.3, 0.3);
const WIN_COLOUR: Color = Color::srgb(0.0, 0.8, 0.2);
const RESOLVE_TIME: f32 = 2.;
const REVEAL_TIME: f32 = 2.;

#[derive(Component, Debug)]
struct Resolve {
    timer: Timer,
}

#[derive(Component, Debug)]
struct Reveal {
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
        app.add_systems(
            OnExit(GameFlow::Reveal),
            (despawn::<Resolve>, despawn::<Reveal>),
        );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    game_data: Res<GameData>,
    rhythm: Res<Rhythm>,
) {
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
                    Text::new(
                        game_data
                            .player_one
                            .choice_selection
                            .get_choice(rhythm.beat)
                            .to_string(),
                    ),
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
                    Text::new(
                        game_data
                            .player_two
                            .choice_selection
                            .get_choice(rhythm.beat)
                            .to_string(),
                    ),
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
    mut commands: Commands,
    rhythm: Res<Rhythm>,
    mut player_one_choice: Query<
        (&mut BackgroundColor, &mut BorderColor, &Children),
        With<PlayerOneChoice>,
    >,
    mut player_two_choice: Query<
        (&mut BackgroundColor, &mut BorderColor, &Children),
        (With<PlayerTwoChoice>, Without<PlayerOneChoice>),
    >,
    mut resolve_query: Query<&mut Resolve>,
    mut reveal_query: Query<&mut Reveal>,
    game_data: Res<GameData>,
    mut game_flow: ResMut<NextState<GameFlow>>,
    time: Res<Time>,
) {
    let Ok(mut player_one) = player_one_choice.get_single_mut() else {
        return;
    };

    let Ok(mut player_two) = player_two_choice.get_single_mut() else {
        return;
    };

    if let Ok(mut resolve) = resolve_query.get_single_mut() {
        resolve.timer.tick(time.delta());
        if resolve.timer.just_finished() {
            let result = game_data.get_result(rhythm.beat);
            println!("{:?}", result);
            match result.outcome {
                Outcome::Draw => {
                    *player_one.0 = LOSE_COLOUR.into();
                    *player_two.0 = LOSE_COLOUR.into();
                }
                Outcome::PlayerOne => {
                    *player_one.0 = WIN_COLOUR.into();
                    *player_two.0 = LOSE_COLOUR.into();
                }
                Outcome::PlayerTwo => {
                    *player_one.0 = LOSE_COLOUR.into();
                    *player_two.0 = WIN_COLOUR.into();
                }
            }

            commands.spawn(Reveal {
                timer: Timer::from_seconds(REVEAL_TIME, TimerMode::Once),
            });
        }
    }
    if let Ok(mut reveal) = reveal_query.get_single_mut() {
        reveal.timer.tick(time.delta());
        // Transition to the next turn
        if reveal.timer.just_finished() {
            if rhythm.can_end_turn() {
                game_flow.set(GameFlow::EndTurn);
            } else {
                game_flow.set(GameFlow::Title);
            }
        }
    }
}
