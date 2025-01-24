use crate::animations::shake_player_sequence;
use crate::config::{FADED_PLAYER, MAX_HEALTH, START_STATE};
use crate::events::{SelectActionEvent, SelectElementEvent};
use crate::globals::GameAssets;
use crate::helper::despawn;
use crate::schedule::GameSet;
use crate::settings::{GameMode, GameSettings};
use crate::state::GameState;
use crate::types::{Action, Choice, Element, Outcome, Player};
use bevy::prelude::*;
use bevy_tweening::Animator;
use std::collections::HashMap;
use std::time::Duration;

const PLAYER_LENGTH: f32 = 1.6;

#[derive(Component)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct ChoiceSelection {
    pub element: Choice,
    pub action: Choice,
}

impl ChoiceSelection {
    pub fn can_double(self) -> bool {
        self.element == Choice::get_complement(&self.action)
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
    pub fn select_element(
        &mut self,
        player: Player,
        choice: Choice,
        writer: &mut EventWriter<SelectElementEvent>,
    ) {
        if self.choice_selection.element != choice {
            self.choice_selection.element = choice;
            writer.send(SelectElementEvent::new(player, choice));
        }
    }
    pub fn select_action(
        &mut self,
        player: Player,
        choice: Choice,
        writer: &mut EventWriter<SelectActionEvent>,
    ) {
        if self.choice_selection.action != choice {
            self.choice_selection.action = choice;
            writer.send(SelectActionEvent::new(player));
        }
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
    pub action: u32,
    pub advantage: Player,
}

impl GameData {
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

    pub fn get_winner(&self) -> Player {
        if self.player_one.health > self.player_two.health {
            Player::One
        } else {
            Player::Two
        }
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
        app.add_systems(OnEnter(START_STATE), setup_game);
        app.add_systems(
            OnEnter(GameState::SelectAction),
            fade_players.in_set(GameSet::Flow),
        );
        app.add_systems(
            OnEnter(GameState::SelectElement),
            fade_players.in_set(GameSet::Flow),
        );
        app.add_systems(Update, shake_players.in_set(GameSet::Flow));
        app.add_systems(
            OnExit(GameState::GameOver),
            (despawn::<PlayerOne>, despawn::<PlayerTwo>).in_set(GameSet::Flow),
        );
    }
}

fn setup_game(
    mut commands: Commands,
    settings: Res<GameSettings>,
    game_assets: Res<GameAssets>,
    mut game_data: ResMut<GameData>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
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
    // Only init controls for Two Player Mode
    if settings.game_mode == GameMode::TwoPlayer {
        game_data.player_two.input = PlayerInput::new(player_two_inputs);
    }

    let texture = game_assets.player_two.neutral.clone();
    let layout = TextureAtlasLayout::from_grid(UVec2::new(302, 286), 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    spawn_player(
        &mut commands,
        PlayerOne,
        Vec3::new(-360.0, -100.0, 0.0),
        AnimationConfig::new(1, 2, 10),
        game_assets.player_one.neutral.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_player(
        &mut commands,
        PlayerOne,
        Vec3::new(360.0, -100.0, 0.0),
        AnimationConfig::new(1, 2, 10),
        game_assets.player_two.neutral.clone(),
        texture_atlas_layout.clone(),
    );
}

fn spawn_player(
    commands: &mut Commands,
    player: impl Component,
    translation: Vec3,
    animation_config: AnimationConfig,
    texture: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
) {
    commands.spawn((
        player,
        Transform::from_translation(translation).with_scale(Vec3::splat(PLAYER_LENGTH)),
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config.first_sprite_index,
            }),
            color: Color::WHITE,
            ..default()
        },
        animation_config,
    ));
}

fn fade_players(
    mut player_one_query: Query<&mut Sprite, With<PlayerOne>>,
    mut player_two_query: Query<&mut Sprite, (With<PlayerTwo>, Without<PlayerOne>)>,
) {
    if let Ok(mut player_one) = player_one_query.get_single_mut() {
        player_one.color = FADED_PLAYER;
    }

    if let Ok(mut player_two) = player_two_query.get_single_mut() {
        player_two.color = FADED_PLAYER;
    }
}

fn shake_players(
    mut commands: Commands,
    mut player_one_query: Query<(Entity, &Transform, &mut Sprite), With<PlayerOne>>,
    mut player_two_query: Query<
        (Entity, &Transform, &mut Sprite),
        (With<PlayerTwo>, Without<PlayerOne>),
    >,
    mut action_reader: EventReader<SelectActionEvent>,
    mut element_reader: EventReader<SelectElementEvent>,
) {
    for event in element_reader.read() {
        match event.player {
            Player::One => {
                if let Ok((entity, transform, mut sprite)) = player_one_query.get_single_mut() {
                    update_player(
                        &event.player,
                        &mut commands,
                        &entity,
                        &transform,
                        &mut sprite,
                    );
                }
            }
            Player::Two => {
                if let Ok((entity, transform, mut sprite)) = player_two_query.get_single_mut() {
                    update_player(
                        &event.player,
                        &mut commands,
                        &entity,
                        &transform,
                        &mut sprite,
                    );
                }
            }
        }
    }
    for event in action_reader.read() {
        match event.player {
            Player::One => {
                if let Ok((entity, transform, mut sprite)) = player_one_query.get_single_mut() {
                    update_player(
                        &event.player,
                        &mut commands,
                        &entity,
                        &transform,
                        &mut sprite,
                    );
                }
            }
            Player::Two => {
                if let Ok((entity, transform, mut sprite)) = player_two_query.get_single_mut() {
                    update_player(
                        &event.player,
                        &mut commands,
                        &entity,
                        &transform,
                        &mut sprite,
                    );
                }
            }
        }
    }
}

fn update_player(
    player: &Player,
    commands: &mut Commands,
    entity: &Entity,
    transform: &Transform,
    sprite: &mut Sprite,
) {
    let shake = shake_player_sequence(transform, *player == Player::One);
    sprite.color = Color::WHITE;
    commands.entity(*entity).insert(Animator::new(shake));
}
