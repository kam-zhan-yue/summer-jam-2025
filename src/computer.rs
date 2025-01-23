use bevy::prelude::*;

use crate::{
    combo::GameData,
    schedule::GameSet,
    settings::{GameMode, GameSettings},
    state::GameState,
    types::{Action, Choice, Element},
};

#[derive(Component, Debug)]
pub struct ComputerPlayer;

pub struct ComputerPlugin;

impl Plugin for ComputerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameStart), setup.in_set(GameSet::Flow));
        app.add_systems(
            OnEnter(GameState::SelectElement),
            select_element.in_set(GameSet::Flow),
        );
        app.add_systems(
            OnEnter(GameState::SelectAction),
            select_action.in_set(GameSet::Flow),
        );
    }
}

fn setup(mut commands: Commands, settings: Res<GameSettings>) {
    if settings.game_mode == GameMode::SinglePlayer {
        commands.spawn(ComputerPlayer);
    }
}

fn select_element(mut game_data: ResMut<GameData>) {
    let random_element = Choice::Element(Element::random());
    game_data.player_two.choice_selection.element = random_element;
}

fn select_action(mut game_data: ResMut<GameData>) {
    let random_action = Choice::Action(Action::random());
    game_data.player_two.choice_selection.action = random_action;
}
