use crate::rhythm::{BeatEvent, ResolveEvent, Rhythm};
use crate::schedule::GameSet;
use crate::settings::GameSettings;
use crate::state::GameState;
use crate::types::{Choice, Location, Player, Tool};
use bevy::prelude::*;
use std::collections::HashMap;

const PLAYER_LENGTH: f32 = 25.;
const PLAYER_ONE_COLOUR: Color = Color::srgb(1., 0., 0.);
const PLAYER_TWO_COLOUR: Color = Color::srgb(0., 1., 0.);
const MAX_HEALTH: i32 = 1;

#[derive(Debug, Default, Copy, Clone)]
pub struct ChoiceSelection {
    tool: Choice,
    location: Choice,
}

#[derive(Component, Debug, Default)]
pub struct PlayerData {
    pub health: i32,
    pub map: HashMap<KeyCode, ChoiceSelection>,
    pub choice_selection: ChoiceSelection,
}

impl PlayerData {
    fn new(map: HashMap<KeyCode, ChoiceSelection>) -> Self {
        Self {
            health: MAX_HEALTH,
            map,
            choice_selection: ChoiceSelection::default(),
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

#[derive(Event, Debug)]
pub struct ChoiceEvent {
    pub choice: Choice,
    pub player: Player,
    pub beat: i32,
}

impl ChoiceEvent {
    fn new(choice: Choice, player: Player, beat: i32) -> Self {
        Self {
            choice,
            player,
            beat,
        }
    }
}

pub struct ComboPlugin;

impl Plugin for ComboPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup_game);
        app.add_systems(
            Update,
            (handle_input, update_combo)
                .chain()
                .in_set(GameSet::Combo)
                .run_if(in_state(GameState::Game)),
        );
        app.add_systems(
            Update,
            resolve_combo
                .in_set(GameSet::Resolve)
                .run_if(in_state(GameState::Game)),
        );
        app.add_event::<ChoiceEvent>();
    }
}

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<GameSettings>,
) {
    let mut player_one_inputs: HashMap<KeyCode, ChoiceSelection> = HashMap::new();
    player_one_inputs.insert(
        KeyCode::KeyA,
        ChoiceSelection {
            tool: Choice::Tool(Tool::Toilet),
            location: Choice::Location(Location::Classroom),
        },
    );
    player_one_inputs.insert(
        KeyCode::KeyS,
        ChoiceSelection {
            tool: Choice::Tool(Tool::Underwear),
            location: Choice::Location(Location::Gymnasium),
        },
    );
    player_one_inputs.insert(
        KeyCode::KeyD,
        ChoiceSelection {
            tool: Choice::Tool(Tool::Lighter),
            location: Choice::Location(Location::Library),
        },
    );

    let mut player_two_inputs: HashMap<KeyCode, ChoiceSelection> = HashMap::new();
    player_two_inputs.insert(
        KeyCode::KeyJ,
        ChoiceSelection {
            tool: Choice::Tool(Tool::Toilet),
            location: Choice::Location(Location::Classroom),
        },
    );
    player_two_inputs.insert(
        KeyCode::KeyK,
        ChoiceSelection {
            tool: Choice::Tool(Tool::Underwear),
            location: Choice::Location(Location::Gymnasium),
        },
    );
    player_two_inputs.insert(
        KeyCode::KeyL,
        ChoiceSelection {
            tool: Choice::Tool(Tool::Lighter),
            location: Choice::Location(Location::Library),
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
    mut player_one_query: Query<&mut PlayerData, With<PlayerOne>>,
    mut player_two_query: Query<&mut PlayerData, (With<PlayerTwo>, Without<PlayerOne>)>,
    input: Res<ButtonInput<KeyCode>>,
    rhythm: Res<Rhythm>,
    mut choice_event_writer: EventWriter<ChoiceEvent>,
) {
    let Ok(mut player_one) = player_one_query.get_single_mut() else {
        return;
    };
    let Ok(mut player_two) = player_two_query.get_single_mut() else {
        return;
    };
    process_input(
        &mut player_one,
        Player::One,
        &input,
        rhythm.beat,
        &mut choice_event_writer,
    );
    process_input(
        &mut player_two,
        Player::Two,
        &input,
        rhythm.beat,
        &mut choice_event_writer,
    );
}

fn process_input(
    player_data: &mut PlayerData,
    player: Player,
    input: &Res<ButtonInput<KeyCode>>,
    beat: i32,
    choice_event_writer: &mut EventWriter<ChoiceEvent>,
) {
    let mut selected_choice = None;
    // Get the selected data
    for (key, choice) in &player_data.map {
        if input.pressed(*key) {
            selected_choice = Some(choice);
            break;
        }
    }

    if let Some(choice) = selected_choice {
        if player_data.choice_selection.tool == Choice::None && beat == 0 {
            player_data.choice_selection.tool = choice.tool;
            choice_event_writer.send(ChoiceEvent::new(choice.tool, player, beat));
        } else if player_data.choice_selection.location == Choice::None && beat == 1 {
            player_data.choice_selection.location = choice.location;
            choice_event_writer.send(ChoiceEvent::new(choice.location, player, beat));
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
                &player.choice_selection.tool
            })
        } else if beat.0 == 2 {
            println!("=========LOCATION=========");
            resolve(&mut player_one, &mut player_two, |player| {
                &player.choice_selection.location
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
    mut game_state: ResMut<NextState<GameState>>,
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
            &player.choice_selection.tool
        });
        let location = resolve(&mut player_one, &mut player_two, |player| {
            &player.choice_selection.location
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

        update_outcome(&mut player_one, &mut player_two, winner, &mut game_state);

        // Reset the player choices
        player_one.choice_selection = ChoiceSelection::default();
        player_two.choice_selection = ChoiceSelection::default();
    }
}

fn update_outcome(
    player_one: &mut PlayerData,
    player_two: &mut PlayerData,
    outcome: Outcome,
    game_state: &mut ResMut<NextState<GameState>>,
) {
    match outcome {
        Outcome::PlayerOne => {
            player_two.health -= 1;
        }
        Outcome::PlayerTwo => {
            player_one.health -= 1;
        }
        Outcome::Draw => (),
    }
    check_end_game(player_one, player_two, game_state);
}

fn check_end_game(
    player_one: &mut PlayerData,
    player_two: &mut PlayerData,
    game_state: &mut ResMut<NextState<GameState>>,
) {
    if player_one.health <= 0 {
        println!("Player Two Wins The Game!");
        game_state.set(GameState::GameOver);
    } else if player_two.health <= 0 {
        println!("Player One Wins The Game!");
        game_state.set(GameState::GameOver);
    }
}
