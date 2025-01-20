use crate::rhythm::BEAT_LIMIT;
use crate::schedule::GameSet;
use crate::state::GameState;
use bevy::prelude::*;

pub struct ChoicesPlugin;

#[derive(Component, Debug, Default)]
struct ChoicesPopup;

#[derive(Component, Debug, Default)]
struct PlayerOneChoices;

#[derive(Component, Debug, Default)]
struct PlayerTwoChoices;

impl Plugin for ChoicesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup);
        app.add_systems(
            Update,
            update_choices
                .in_set(GameSet::Ui)
                .run_if(in_state(GameState::Game)),
        );
    }
}

fn setup(mut commands: Commands) {
    // Root Node
    commands
        .spawn((
            ChoicesPopup,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            spawn_player_choices(
                parent,
                PlayerOneChoices,
                JustifyContent::Start,
                FlexDirection::Row,
            );
            spawn_player_choices(
                parent,
                PlayerTwoChoices,
                JustifyContent::End,
                FlexDirection::RowReverse,
            );
        });
}

fn spawn_player_choices(
    parent: &mut ChildBuilder,
    player: impl Component,
    justify_content: JustifyContent,
    flex_direction: FlexDirection,
) {
    parent
        .spawn((
            player,
            Node {
                width: Val::Percent(50.0),
                justify_content,
                flex_direction,
                ..default()
            },
        ))
        .with_children(|parent| {
            for i in 0..BEAT_LIMIT {
                parent
                    .spawn((
                        Node {
                            width: Val::Px(75.0),
                            height: Val::Px(75.0),
                            border: UiRect::all(Val::Px(5.0)),
                            align_items: AlignItems::Center,
                            align_content: AlignContent::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BorderColor(Color::BLACK),
                        BorderRadius::MAX,
                        BackgroundColor(Color::WHITE),
                    ))
                    .with_child((Text::new(i.to_string()), TextColor(Color::BLACK)));
            }
        });
}

fn update_choices() {}
