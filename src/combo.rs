use crate::animations::shake_player_sequence;
use crate::config::{FADED_PLAYER, MAX_HEALTH, START_STATE};
use crate::events::{SelectActionEvent, SelectElementEvent};
use crate::globals::{GameAssets, PlayerAsset};
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
    pub starting_pos: Vec3,
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
            starting_pos: Vec3::default(),
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
            (fade_players, reset_player_elements).in_set(GameSet::Flow),
        );
        app.add_systems(
            Update,
            (animate_players, shake_players, update_player_elements).in_set(GameSet::Flow),
        );
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

    game_data.player_one.starting_pos = Vec3::new(-360.0, -100.0, 0.0);
    game_data.player_two.starting_pos = Vec3::new(360.0, -100.0, 0.0);

    let layout = TextureAtlasLayout::from_grid(UVec2::new(302, 286), 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    spawn_player(
        &mut commands,
        PlayerOne,
        game_data.player_one.starting_pos,
        AnimationConfig::new(0, 1, 10),
        game_assets.player_one.neutral.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_player(
        &mut commands,
        PlayerTwo,
        game_data.player_two.starting_pos,
        AnimationConfig::new(0, 1, 10),
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

fn animate_players(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in query.iter_mut() {
        config.frame_timer.tick(time.delta());

        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == config.last_sprite_index {
                    atlas.index = config.first_sprite_index;
                } else {
                    atlas.index += 1;
                }
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
    }
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

fn update_player_elements(
    mut player_one_query: Query<&mut Sprite, With<PlayerOne>>,
    mut player_two_query: Query<&mut Sprite, (With<PlayerTwo>, Without<PlayerOne>)>,
    mut element_reader: EventReader<SelectElementEvent>,
    game_assets: Res<GameAssets>,
) {
    for event in element_reader.read() {
        match event.player {
            Player::One => {
                if let Ok(mut sprite) = player_one_query.get_single_mut() {
                    update_player_element(&mut sprite, &event.element, &game_assets.player_one);
                }
            }
            Player::Two => {
                if let Ok(mut sprite) = player_two_query.get_single_mut() {
                    update_player_element(&mut sprite, &event.element, &game_assets.player_two);
                }
            }
        }
    }
}

fn update_player_element(sprite: &mut Sprite, element: &Choice, player_asset: &PlayerAsset) {
    match element {
        Choice::Element(Element::Fire) => sprite.image = player_asset.fire.clone(),
        Choice::Element(Element::Water) => sprite.image = player_asset.water.clone(),
        Choice::Element(Element::Grass) => sprite.image = player_asset.grass.clone(),
        _ => sprite.image = player_asset.neutral.clone(),
    }
}

fn reset_player_elements(
    mut player_one_query: Query<&mut Sprite, With<PlayerOne>>,
    mut player_two_query: Query<&mut Sprite, (With<PlayerTwo>, Without<PlayerOne>)>,
    game_assets: Res<GameAssets>,
) {
    if let Ok(mut sprite) = player_one_query.get_single_mut() {
        update_player_element(&mut sprite, &Choice::None, &game_assets.player_one);
    }

    if let Ok(mut sprite) = player_two_query.get_single_mut() {
        update_player_element(&mut sprite, &Choice::None, &game_assets.player_two);
    }
}

fn shake_players(
    mut commands: Commands,
    mut player_one_query: Query<(Entity, &mut Sprite), With<PlayerOne>>,
    mut player_two_query: Query<(Entity, &mut Sprite), (With<PlayerTwo>, Without<PlayerOne>)>,
    mut action_reader: EventReader<SelectActionEvent>,
    mut element_reader: EventReader<SelectElementEvent>,
    game_data: Res<GameData>,
) {
    for event in element_reader.read() {
        match event.player {
            Player::One => {
                if let Ok((entity, mut sprite)) = player_one_query.get_single_mut() {
                    shake_player(
                        &event.player,
                        &mut commands,
                        &entity,
                        &mut sprite,
                        &game_data.player_one.starting_pos,
                    );
                }
            }
            Player::Two => {
                if let Ok((entity, mut sprite)) = player_two_query.get_single_mut() {
                    shake_player(
                        &event.player,
                        &mut commands,
                        &entity,
                        &mut sprite,
                        &game_data.player_two.starting_pos,
                    );
                }
            }
        }
    }
    for event in action_reader.read() {
        match event.player {
            Player::One => {
                if let Ok((entity, mut sprite)) = player_one_query.get_single_mut() {
                    shake_player(
                        &event.player,
                        &mut commands,
                        &entity,
                        &mut sprite,
                        &game_data.player_one.starting_pos,
                    );
                }
            }
            Player::Two => {
                if let Ok((entity, mut sprite)) = player_two_query.get_single_mut() {
                    shake_player(
                        &event.player,
                        &mut commands,
                        &entity,
                        &mut sprite,
                        &game_data.player_two.starting_pos,
                    );
                }
            }
        }
    }
}

fn shake_player(
    player: &Player,
    commands: &mut Commands,
    entity: &Entity,
    sprite: &mut Sprite,
    original_pos: &Vec3,
) {
    let shake = shake_player_sequence(original_pos, *player == Player::One);
    sprite.color = Color::WHITE;
    commands.entity(*entity).insert(Animator::new(shake));
}
