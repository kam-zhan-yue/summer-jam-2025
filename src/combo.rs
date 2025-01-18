use crate::schedule::GameSet;
use bevy::prelude::*;

const MAX_HEALTH: i32 = 100;

#[derive(Component, Debug, Default)]
pub struct Combo {
    pub inputs: Vec<char>,
}

#[derive(Component, Debug)]
pub struct Health(i32);

impl Default for Health {
    fn default() -> Self {
        Health(MAX_HEALTH)
    }
}

#[derive(Component, Debug, Default)]
#[require(Health, Combo)]
pub struct Player;

#[derive(Resource, Debug)]
pub struct Game {
    player_one: Player,
    player_two: Player,
}

impl Default for Game {
    fn default() -> Game {
        Self {
            player_one: Player::default(),
            player_two: Player::default(),
        }
    }
}

pub struct ComboPlugin;

impl Plugin for ComboPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Game>();
        app.add_systems(
            Update,
            (handle_input, update_combo).chain().in_set(GameSet::Combo),
        );
        app.add_systems(Update, resolve_combo.in_set(GameSet::Resolve));
    }
}

fn handle_input() {}

fn update_combo() {}

fn resolve_combo() {}
