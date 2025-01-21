use crate::schedule::GameSet;
use crate::settings::GameSettings;
use crate::state::{GameFlow, GameState};
use crate::types::{Action, Choice, Element, Outcome, Player};
use bevy::prelude::*;
use std::collections::HashMap;

const PLAYER_LENGTH: f32 = 1.4;
pub const MAX_HEALTH: i32 = 3;

#[derive(Debug, Default, Copy, Clone)]
pub struct ChoiceSelection {
    pub element: Choice,
    pub action: Choice,
}

impl ChoiceSelection {
    pub fn get_choice(self, beat: i32) -> Choice {
        match beat {
            1 => self.element,
            2 => self.action,
            _ => Choice::None,
        }
    }
}

#[derive(Debug, Default)]
pub struct PlayerInput {
    pub map: HashMap<KeyCode, ChoiceSelection>,
}

impl PlayerInput {
    fn new(map: HashMap<KeyCode, ChoiceSelection>) -> Self {
        Self { map }
    }
}

#[derive(Debug)]
pub struct PlayerData {
    pub choice_selection: ChoiceSelection,
    pub health: i32,
    pub input: PlayerInput,
}

impl PlayerData {
    fn get_choice(&self, beat: i32) -> Choice {
        self.choice_selection.get_choice(beat)
    }
}

impl Default for PlayerData {
    fn default() -> Self {
        Self {
            choice_selection: ChoiceSelection::default(),
            input: PlayerInput::default(),
            health: MAX_HEALTH,
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct GameData {
    pub player_one: PlayerData,
    pub player_two: PlayerData,
}

impl GameData {
    pub fn get_choice(&self, player: Player, beat: i32) -> Choice {
        match player {
            Player::One => self.player_one.get_choice(beat),
            Player::Two => self.player_two.get_choice(beat),
        }
    }

    pub fn get_element(&self, player: Player) -> Choice {
        match player {
            Player::One => self.player_one.choice_selection.element,
            Player::Two => self.player_two.choice_selection.element,
        }
    }

    pub fn get_action(&self, player: Player) -> Choice {
        match player {
            Player::One => self.player_one.choice_selection.action,
            Player::Two => self.player_two.choice_selection.action,
        }
    }

    pub fn reset(&mut self) {
        self.player_one.choice_selection = ChoiceSelection::default();
        self.player_two.choice_selection = ChoiceSelection::default();
    }

    pub fn get_winner(&self) -> Outcome {
        println!("=========RESOLVE=========");
        let mut map: HashMap<Outcome, i32> = HashMap::new();

        let tool = self.resolve(|player| player.choice_selection.action);
        let location = self.resolve(|player| player.choice_selection.action);

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
        return winner;
    }

    pub fn get_action_result(&self) -> ResolveResult {
        self.resolve(|player| player.choice_selection.action)
    }

    pub fn get_result(&self, beat: i32) -> ResolveResult {
        let result = if beat == 1 {
            println!("=========Element=========");
            self.resolve(|player| player.choice_selection.element)
        } else if beat == 2 {
            println!("=========Tool=========");
            self.resolve(|player| player.choice_selection.action)
        } else {
            ResolveResult {
                outcome: Outcome::Draw,
                choice: Choice::None,
            }
        };
        return result;
    }

    pub fn resolve(&self, get_choice: fn(&PlayerData) -> Choice) -> ResolveResult {
        let choice_one = get_choice(&self.player_one);
        let choice_two = get_choice(&self.player_two);
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

    pub fn can_end_game(&self) -> bool {
        self.player_one.health <= 0 || self.player_two.health <= 0
    }
}

#[derive(Component, Debug)]
pub struct PlayerOne;

#[derive(Component, Debug)]
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
            OnEnter(GameFlow::EndTurn),
            enter_resolve.in_set(GameSet::Resolve),
        );
        app.add_event::<ChoiceEvent>();
    }
}

fn setup_game(
    mut commands: Commands,
    settings: Res<GameSettings>,
    asset_server: ResMut<AssetServer>,
    mut game_data: ResMut<GameData>,
) {
    let mut player_one_inputs: HashMap<KeyCode, ChoiceSelection> = HashMap::new();
    player_one_inputs.insert(
        KeyCode::KeyA,
        ChoiceSelection {
            action: Choice::Action(Action::Toilet),
            element: Choice::Element(Element::Water),
        },
    );
    player_one_inputs.insert(
        KeyCode::KeyS,
        ChoiceSelection {
            action: Choice::Action(Action::Underwear),
            element: Choice::Element(Element::Grass),
        },
    );
    player_one_inputs.insert(
        KeyCode::KeyD,
        ChoiceSelection {
            action: Choice::Action(Action::Hand),
            element: Choice::Element(Element::Fire),
        },
    );

    let mut player_two_inputs: HashMap<KeyCode, ChoiceSelection> = HashMap::new();
    player_two_inputs.insert(
        KeyCode::KeyJ,
        ChoiceSelection {
            action: Choice::Action(Action::Toilet),
            element: Choice::Element(Element::Water),
        },
    );
    player_two_inputs.insert(
        KeyCode::KeyK,
        ChoiceSelection {
            action: Choice::Action(Action::Underwear),
            element: Choice::Element(Element::Grass),
        },
    );
    player_two_inputs.insert(
        KeyCode::KeyL,
        ChoiceSelection {
            action: Choice::Action(Action::Hand),
            element: Choice::Element(Element::Fire),
        },
    );

    game_data.player_one.input = PlayerInput::new(player_one_inputs);
    game_data.player_two.input = PlayerInput::new(player_two_inputs);

    commands.spawn((
        Transform::from_xyz(-300., -100., 0.).with_scale(Vec3::splat(PLAYER_LENGTH)),
        PlayerOne,
        Sprite::from_image(asset_server.load("sprites/stick_left.png")),
    ));

    commands.spawn((
        Transform::from_xyz(300., -100., 0.).with_scale(Vec3::splat(PLAYER_LENGTH)),
        PlayerTwo,
        Sprite::from_image(asset_server.load("sprites/stick_right.png")),
    ));
}

fn enter_resolve(mut game_state: ResMut<NextState<GameState>>, mut game_data: ResMut<GameData>) {
    let winner = &game_data.get_winner();

    update_outcome(&mut game_data, winner, &mut game_state);

    game_data.reset();
}

fn update_outcome(
    game_data: &mut ResMut<GameData>,
    outcome: &Outcome,
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
