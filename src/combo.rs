use crate::helper::despawn;
use crate::schedule::GameSet;
use crate::settings::GameSettings;
use crate::state::GameState;
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
    pub fn can_double(self) -> bool {
        match (self.element, self.action) {
            (Choice::Element(Element::Fire), Choice::Action(Action::Hand)) => true,
            (Choice::Element(Element::Water), Choice::Action(Action::Toilet)) => true,
            (Choice::Element(Element::Grass), Choice::Action(Action::Underwear)) => true,
            _ => false,
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
    pub action: u32,
    pub advantage: Player,
}

impl GameData {
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
        self.player_one = PlayerData::default();
        self.player_two = PlayerData::default();
        self.action = 0;
    }

    pub fn reset_choice(&mut self) {
        self.player_one.choice_selection = ChoiceSelection::default();
        self.player_two.choice_selection = ChoiceSelection::default();
    }

    pub fn get_action_result(&self) -> ResolveResult {
        self.resolve(|player| player.choice_selection.action)
    }

    pub fn process_turn(&mut self) {
        // Increment Action by One
        self.action += 1;
        // Update Healths
        let result = self.get_action_result();
        match result.outcome {
            Outcome::PlayerOne => {
                if self.player_one.choice_selection.can_double() {
                    self.player_two.health -= 2;
                } else {
                    self.player_two.health -= 1;
                }
            }
            Outcome::PlayerTwo => {
                if self.player_two.choice_selection.can_double() {
                    self.player_one.health -= 2;
                } else {
                    self.player_one.health -= 1;
                }
            }
            Outcome::Draw => (),
        }
        // Reset Choices
        self.reset_choice();
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

#[derive(Debug)]
pub struct ResolveResult {
    pub outcome: Outcome,
    pub choice: Choice,
}

pub struct ComboPlugin;

impl Plugin for ComboPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameData>();
        app.add_systems(OnEnter(GameState::GameStart), setup_game);
        app.add_systems(
            OnExit(GameState::GameOver),
            (despawn::<PlayerOne>, despawn::<PlayerTwo>).in_set(GameSet::Flow),
        );
    }
}

fn setup_game(
    mut commands: Commands,
    settings: Res<GameSettings>,
    asset_server: ResMut<AssetServer>,
    mut game_data: ResMut<GameData>,
) {
    game_data.reset();
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
