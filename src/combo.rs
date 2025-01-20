use crate::rhythm::{BeatEvent, ResolveEvent, Rhythm};
use crate::schedule::GameSet;
use crate::settings::GameSettings;
use crate::state::GameState;
use crate::types::{Location, Tool};
use bevy::prelude::*;
use std::collections::HashMap;

const PLAYER_LENGTH: f32 = 25.;
const PLAYER_ONE_COLOUR: Color = Color::srgb(1., 0., 0.);
const PLAYER_TWO_COLOUR: Color = Color::srgb(0., 1., 0.);
const MAX_HEALTH: i32 = 100;

#[derive(Debug, Default)]
pub struct Choice {
    pub tool: Tool,
    pub location: Location,
}

#[derive(Component, Debug, Default)]
pub struct PlayerData {
    pub health: i32,
    pub map: HashMap<KeyCode, Choice>,
    pub choice: Choice,
}

impl PlayerData {
    fn new(map: HashMap<KeyCode, Choice>) -> Self {
        Self {
            health: MAX_HEALTH,
            map,
            choice: Choice::default(),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
enum Outcome {
    PlayerOne,
    PlayerTwo,
    Draw,
}

#[derive(Component, Debug)]
#[require(PlayerData)]
pub struct PlayerOne;

#[derive(Component, Debug)]
#[require(PlayerData)]
pub struct PlayerTwo;

pub struct ComboPlugin;

impl Plugin for ComboPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup_game)
            .add_systems(
                Update,
                (handle_input, update_combo)
                    .chain()
                    .in_set(GameSet::Combo)
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(
                Update,
                resolve_combo
                    .in_set(GameSet::Resolve)
                    .run_if(in_state(GameState::Game)),
            );
    }
}

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<GameSettings>,
) {
    let mut player_one_inputs: HashMap<KeyCode, Choice> = HashMap::new();
    player_one_inputs.insert(
        KeyCode::KeyA,
        Choice {
            tool: Tool::Toilet,
            location: Location::Classroom,
        },
    );
    player_one_inputs.insert(
        KeyCode::KeyS,
        Choice {
            tool: Tool::Underwear,
            location: Location::Gymnasium,
        },
    );
    player_one_inputs.insert(
        KeyCode::KeyD,
        Choice {
            tool: Tool::Lighter,
            location: Location::Library,
        },
    );

    let mut player_two_inputs: HashMap<KeyCode, Choice> = HashMap::new();
    player_two_inputs.insert(
        KeyCode::KeyJ,
        Choice {
            tool: Tool::Toilet,
            location: Location::Classroom,
        },
    );
    player_two_inputs.insert(
        KeyCode::KeyK,
        Choice {
            tool: Tool::Underwear,
            location: Location::Gymnasium,
        },
    );
    player_two_inputs.insert(
        KeyCode::KeyL,
        Choice {
            tool: Tool::Lighter,
            location: Location::Library,
        },
    );

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(PLAYER_ONE_COLOUR)),
        Transform::from_xyz(-100., 0., 0.).with_scale(Vec3::splat(PLAYER_LENGTH)),
        PlayerOne,
        PlayerData::new(player_one_inputs),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::default())),
        MeshMaterial2d(materials.add(PLAYER_TWO_COLOUR)),
        Transform::from_xyz(100., 0., 0.).with_scale(Vec3::splat(PLAYER_LENGTH)),
        PlayerTwo,
        PlayerData::new(player_two_inputs),
    ));
}

fn handle_input(
    mut query: Query<&mut PlayerData>,
    input: Res<ButtonInput<KeyCode>>,
    rhythm: Res<Rhythm>,
) {
    for mut player in query.iter_mut() {
        let mut selected_choice = None;
        // Get the selected data
        for (key, choice) in &player.map {
            if input.pressed(*key) {
                selected_choice = Some(choice);
                break;
            }
        }

        if let Some(choice) = selected_choice {
            if player.choice.tool == Tool::None && rhythm.beat == 0 {
                player.choice.tool = choice.tool;
                // println!("{:?}", player.choice.tool);
            } else if player.choice.location == Location::None && rhythm.beat == 1 {
                player.choice.location = choice.location;
                // println!("{:?}", player.choice.location);
            }
        }
    }
}

fn update_combo(
    mut player_one_query: Query<&mut PlayerData, With<PlayerOne>>,
    mut player_two_query: Query<&mut PlayerData, (With<PlayerTwo>, Without<PlayerOne>)>,
    mut beat_event_reader: EventReader<BeatEvent>,
) {
    let Ok(mut player_one) = player_one_query.get_single_mut() else {
        return;
    };
    let Ok(mut player_two) = player_two_query.get_single_mut() else {
        return;
    };
    for beat in beat_event_reader.read() {
        let outcome = if beat.0 == 1 {
            println!("=========TOOL=========");
            resolve(&mut player_one, &mut player_two, |player| {
                &player.choice.tool
            })
        } else if beat.0 == 2 {
            println!("=========LOCATION=========");
            resolve(&mut player_one, &mut player_two, |player| {
                &player.choice.location
            })
        } else {
            Outcome::Draw
        };

        match outcome {
            Outcome::PlayerOne => println!("Player One Wins"),
            Outcome::PlayerTwo => println!("Player Two Wins"),
            Outcome::Draw => println!("Draw"),
        }
    }
}

fn resolve<T: std::fmt::Debug + PartialOrd>(
    player_one: &mut PlayerData,
    player_two: &mut PlayerData,
    get_choice: fn(&PlayerData) -> &T,
) -> Outcome {
    let choice_one = get_choice(player_one);
    let choice_two = get_choice(player_two);
    println!("Player One: {:?} Player Two: {:?} ", choice_one, choice_two);
    if choice_one > choice_two {
        Outcome::PlayerOne
    } else if choice_one < choice_two {
        Outcome::PlayerTwo
    } else {
        Outcome::Draw
    }
}

fn resolve_combo(
    mut player_one_query: Query<&mut PlayerData, With<PlayerOne>>,
    mut player_two_query: Query<&mut PlayerData, (With<PlayerTwo>, Without<PlayerOne>)>,
    mut resolve_event_reader: EventReader<ResolveEvent>,
) {
    for _ in resolve_event_reader.read() {
        let Ok(mut player_one) = player_one_query.get_single_mut() else {
            return;
        };
        let Ok(mut player_two) = player_two_query.get_single_mut() else {
            return;
        };

        println!("=========RESOLVE=========");
        let mut map: HashMap<Outcome, i32> = HashMap::new();

        let tool = resolve(&mut player_one, &mut player_two, |player| {
            &player.choice.tool
        });
        let location = resolve(&mut player_one, &mut player_two, |player| {
            &player.choice.location
        });

        *map.entry(tool).or_insert(0) += 1;
        *map.entry(location).or_insert(0) += 1;

        let player_one_wins = map.get(&Outcome::PlayerOne).unwrap_or(&0);
        let player_two_wins = map.get(&Outcome::PlayerTwo).unwrap_or(&0);

        let winner = if player_one_wins > player_two_wins {
            Outcome::PlayerOne
        } else if player_one_wins < player_two_wins {
            Outcome::PlayerTwo
        } else {
            Outcome::Draw
        };
        println!("Overall Winner is: {:?}", winner);

        update_outcome(&mut player_one, &mut player_two, winner);

        // Reset the player choices
        player_one.choice = Choice::default();
        player_two.choice = Choice::default();
    }
}

fn update_outcome(player_one: &mut PlayerData, player_two: &mut PlayerData, outcome: Outcome) {
    match outcome {
        Outcome::PlayerOne => {
            player_two.health -= 1;
        }
        Outcome::PlayerTwo => {
            player_one.health -= 1;
        }
        Outcome::Draw => (),
    }
    check_end_game(player_one, player_two);
}

fn check_end_game(player_one: &mut PlayerData, player_two: &mut PlayerData) {
    if player_one.health <= 0 {
        println!("Player Two Wins The Game!");
    } else if player_two.health <= 0 {
        println!("Player One Wins The Game!");
    }
}
