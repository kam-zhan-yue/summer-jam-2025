use crate::rhythm::{BeatEvent, ResolveEvent, Rhythm};
use crate::schedule::GameSet;
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

#[derive(Component, Debug)]
pub struct Health(i32);

impl Default for Health {
    fn default() -> Self {
        Health(MAX_HEALTH)
    }
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

#[derive(Component, Debug)]
#[require(PlayerData)]
pub struct PlayerOne;

#[derive(Component, Debug)]
#[require(PlayerData)]
pub struct PlayerTwo;

pub struct ComboPlugin;

impl Plugin for ComboPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_game)
            .add_systems(
                Update,
                (handle_input, update_combo).chain().in_set(GameSet::Combo),
            )
            .add_systems(Update, resolve_combo.in_set(GameSet::Resolve));
    }
}

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // game: &mut ResMut<Game>,
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
            if player.choice.tool == Tool::None && rhythm.beat == 1 {
                println!("{:?}", choice.tool);
                player.choice.tool = choice.tool;
            } else if player.choice.location == Location::None && rhythm.beat == 2 {
                println!("{:?}", choice.location);
                player.choice.location = choice.location
            }
        }
    }
}

fn update_combo(
    mut player_one_query: Query<&mut PlayerData, With<PlayerOne>>,
    mut player_two_query: Query<&mut PlayerData, (With<PlayerTwo>, Without<PlayerOne>)>,
    mut beat_event_reader: EventReader<BeatEvent>,
) {
    if beat_event_reader.len() == 0 {
        return;
    }
    let Ok(mut player_one) = player_one_query.get_single_mut() else {
        return;
    };
    let Ok(mut player_two) = player_two_query.get_single_mut() else {
        return;
    };
    for beat in beat_event_reader.read() {
        match beat.0 {
            1 => resolve_tool(&mut player_one, &mut player_two),
            2 => resolve_location(&mut player_one, &mut player_two),
            _ => (),
        }
    }
}

fn resolve_tool(player_one: &mut PlayerData, player_two: &mut PlayerData) {
    let tool_one = player_one.choice.tool;
    let tool_two = player_two.choice.tool;
    print!("Player One: {:?} Player Two: {:?} ", tool_one, tool_two);
    if tool_one > tool_two {
        print!("Player One Wins");
    } else if tool_one < tool_two {
        print!("Player Two Wins");
    } else {
        print!("Draw");
    }
    print!("\n");
}

fn resolve_location(player_one: &mut PlayerData, player_two: &mut PlayerData) {
    let location_one = player_one.choice.location;
    let location_two = player_two.choice.location;
    print!(
        "Player One: {:?} Player Two: {:?} ",
        location_one, location_two
    );
    if location_one > location_two {
        print!("Player One Wins");
    } else if location_one < location_two {
        print!("Player Two Wins");
    } else {
        print!("Draw");
    }
    print!("\n");
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
        // Reset the player choices
        player_one.choice = Choice::default();
        player_two.choice = Choice::default();
        println!("Resolve!");
    }
}
