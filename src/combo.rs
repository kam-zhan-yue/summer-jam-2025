use crate::rhythm::Rhythm;
use crate::schedule::GameSet;
use crate::settings::GameSettings;
use crate::state::{GameFlow, GameState};
use crate::types::{Choice, Location, Outcome, Player, Tool};
use bevy::prelude::*;
use std::collections::HashMap;

const PLAYER_LENGTH: f32 = 1.4;
const MAX_HEALTH: i32 = 1;

#[derive(Debug, Default, Copy, Clone)]
pub struct ChoiceSelection {
    pub tool: Choice,
    pub location: Choice,
}

impl ChoiceSelection {
    pub fn get_choice(self, beat: i32) -> Choice {
        match beat {
            1 => self.tool,
            2 => self.location,
            _ => Choice::None,
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct PlayerInput {
    pub map: HashMap<KeyCode, ChoiceSelection>,
}

impl PlayerInput {
    fn new(map: HashMap<KeyCode, ChoiceSelection>) -> Self {
        Self { map }
    }
}

#[derive(Debug, Default)]
pub struct PlayerData {
    pub choice_selection: ChoiceSelection,
    pub health: i32,
}

#[derive(Resource, Debug, Default)]
pub struct GameData {
    pub player_one: PlayerData,
    pub player_two: PlayerData,
}

#[derive(Component, Debug)]
#[require(PlayerInput)]
pub struct PlayerOne;

#[derive(Component, Debug)]
#[require(PlayerInput)]
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

#[derive(Event, Debug)]
pub struct ResolveEvent(ResolveResult);

impl ResolveEvent {
    fn new(result: ResolveResult) -> Self {
        Self(result)
    }
}

#[derive(Debug)]
pub struct ResolveResult {
    pub outcome: Outcome,
    pub choice: Choice,
}

impl ResolveResult {
    fn new(outcome: Outcome, choice: Choice) -> Self {
        Self { outcome, choice }
    }
}

pub struct ComboPlugin;

impl Plugin for ComboPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameData>();
        app.add_systems(OnEnter(GameState::Game), setup_game);
        app.add_systems(
            Update,
            handle_input
                .in_set(GameSet::Resolve)
                .run_if(in_state(GameState::Game)),
        );
        app.add_systems(
            OnEnter(GameFlow::Reveal),
            enter_reveal.in_set(GameSet::Resolve),
        );
        app.add_systems(
            OnEnter(GameFlow::EndTurn),
            enter_resolve.in_set(GameSet::Resolve),
        );
        app.add_event::<ChoiceEvent>();
        app.add_event::<ResolveEvent>();
    }
}

fn setup_game(
    mut commands: Commands,
    settings: Res<GameSettings>,
    asset_server: ResMut<AssetServer>,
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
        Transform::from_xyz(-300., -100., 0.).with_scale(Vec3::splat(PLAYER_LENGTH)),
        PlayerOne,
        PlayerInput::new(player_one_inputs),
        Sprite::from_image(asset_server.load("sprites/stick_left.png")),
    ));

    commands.spawn((
        Transform::from_xyz(300., -100., 0.).with_scale(Vec3::splat(PLAYER_LENGTH)),
        PlayerTwo,
        PlayerInput::new(player_two_inputs),
        Sprite::from_image(asset_server.load("sprites/stick_right.png")),
    ));
}

fn handle_input(
    mut player_one_query: Query<&mut PlayerInput, With<PlayerOne>>,
    mut player_two_query: Query<&mut PlayerInput, (With<PlayerTwo>, Without<PlayerOne>)>,
    input: Res<ButtonInput<KeyCode>>,
    rhythm: Res<Rhythm>,
    mut game_data: ResMut<GameData>,
    mut choice_event_writer: EventWriter<ChoiceEvent>,
) {
    let Ok(player_one_input) = player_one_query.get_single_mut() else {
        return;
    };
    let Ok(player_two_input) = player_two_query.get_single_mut() else {
        return;
    };
    process_input(
        &player_one_input,
        &mut game_data.player_one,
        Player::One,
        &input,
        rhythm.beat,
        &mut choice_event_writer,
    );
    process_input(
        &player_two_input,
        &mut game_data.player_two,
        Player::Two,
        &input,
        rhythm.beat,
        &mut choice_event_writer,
    );
}

fn process_input(
    player_input: &PlayerInput,
    player_data: &mut PlayerData,
    player: Player,
    input: &Res<ButtonInput<KeyCode>>,
    beat: i32,
    choice_event_writer: &mut EventWriter<ChoiceEvent>,
) {
    let mut selected_choice = None;
    // Get the selected data
    for (key, choice) in &player_input.map {
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

pub fn resolve(game_data: &mut GameData, get_choice: fn(&PlayerData) -> Choice) -> ResolveResult {
    let choice_one = get_choice(&game_data.player_one);
    let choice_two = get_choice(&game_data.player_two);
    println!("Player One: {:?} Player Two: {:?} ", choice_one, choice_two);
    if choice_one > choice_two {
        ResolveResult {
            outcome: Outcome::PlayerOne,
            choice: choice_one.clone(),
        }
    } else if choice_one < choice_two {
        ResolveResult {
            outcome: Outcome::PlayerTwo,
            choice: choice_two.clone(),
        }
    } else {
        ResolveResult {
            outcome: Outcome::Draw,
            choice: Choice::None,
        }
    }
}

fn enter_reveal(
    rhythm: Res<Rhythm>,
    mut resolve_event_writer: EventWriter<ResolveEvent>,
    mut game_data: ResMut<GameData>,
) {
    let result = if rhythm.beat == 1 {
        println!("=========TOOL=========");
        resolve(&mut game_data, |player| player.choice_selection.tool)
    } else if rhythm.beat == 2 {
        println!("=========LOCATION=========");
        resolve(&mut game_data, |player| player.choice_selection.location)
    } else {
        ResolveResult {
            outcome: Outcome::Draw,
            choice: Choice::None,
        }
    };

    match result.outcome {
        Outcome::PlayerOne => println!("Player One Wins"),
        Outcome::PlayerTwo => println!("Player Two Wins"),
        Outcome::Draw => println!("Draw"),
    }

    // resolve_event_writer.send(ResolveEvent::new(result));
}

fn enter_resolve(mut game_state: ResMut<NextState<GameState>>, mut game_data: ResMut<GameData>) {
    println!("=========RESOLVE=========");
    let mut map: HashMap<Outcome, i32> = HashMap::new();

    let tool = resolve(&mut game_data, |player| player.choice_selection.tool);
    let location = resolve(&mut game_data, |player| player.choice_selection.tool);

    *map.entry(tool.outcome).or_insert(0) += 1;
    *map.entry(location.outcome).or_insert(0) += 1;

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

    update_outcome(&mut game_data, winner, &mut game_state);

    // Reset the player choices
    game_data.player_one.choice_selection = ChoiceSelection::default();
    game_data.player_two.choice_selection = ChoiceSelection::default();
}

fn update_outcome(
    game_data: &mut ResMut<GameData>,
    outcome: Outcome,
    game_state: &mut ResMut<NextState<GameState>>,
) {
    match outcome {
        Outcome::PlayerOne => {
            game_data.player_two.health -= 1;
        }
        Outcome::PlayerTwo => {
            game_data.player_one.health -= 1;
        }
        Outcome::Draw => (),
    }
    check_end_game(game_data, game_state);
}

fn check_end_game(game_data: &mut GameData, game_state: &mut ResMut<NextState<GameState>>) {
    if game_data.player_one.health <= 0 {
        println!("Player Two Wins The Game!");
        game_state.set(GameState::GameOver);
    } else if game_data.player_two.health <= 0 {
        println!("Player One Wins The Game!");
        game_state.set(GameState::GameOver);
    }
}
