use crate::combo::ChoiceEvent;
use crate::rhythm::BEAT_LIMIT;
use crate::schedule::GameSet;
use crate::state::GameState;
use crate::types::{Choice, Player};
use bevy::prelude::*;

pub struct ChoicesPlugin;

#[derive(Component, Debug, Default)]
struct ChoicesPopup;

#[derive(Component, Debug, Default)]
struct PlayerOneChoices;

#[derive(Component, Debug, Default)]
struct PlayerTwoChoices;

#[derive(Component, Debug, Default)]
struct ChoicePopupItem;

impl Plugin for ChoicesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup);
        app.add_systems(
            Update,
            read_choices
                .in_set(GameSet::Ui)
                .run_if(in_state(GameState::Game)),
        );
    }
}

fn setup(mut commands: Commands) {
    // Root Node
    commands
        .spawn((
            Name::new("Choices Popup"),
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
                Name::new("Player One Choices"),
                parent,
                PlayerOneChoices,
                JustifyContent::Start,
                FlexDirection::Row,
            );
            spawn_player_choices(
                Name::new("Player Two Choices"),
                parent,
                PlayerTwoChoices,
                JustifyContent::End,
                FlexDirection::RowReverse,
            );
        });
}

fn spawn_player_choices(
    name: impl Component,
    parent: &mut ChildBuilder,
    player: impl Component,
    justify_content: JustifyContent,
    flex_direction: FlexDirection,
) {
    parent
        .spawn((
            name,
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
                        ChoicePopupItem,
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

fn read_choices(
    mut choice_event_reader: EventReader<ChoiceEvent>,
    player_one_choices: Query<&Children, With<PlayerOneChoices>>,
    player_two_choices: Query<&Children, With<PlayerTwoChoices>>,
    choice_popup_items: Query<&Children, With<ChoicePopupItem>>,
    mut text_query: Query<&mut Text>,
) {
    let Ok(player_one_children) = player_one_choices.get_single() else {
        return;
    };
    let Ok(player_two_children) = player_two_choices.get_single() else {
        return;
    };
    for choice in choice_event_reader.read() {
        let children = match choice.player {
            Player::One => player_one_children,
            Player::Two => player_two_children,
        };
        if let Some(&choice_popup_item) = children.get(choice.beat as usize) {
            if let Ok(popup_children) = choice_popup_items.get(choice_popup_item) {
                if let Some(&child) = popup_children.get(0) {
                    if let Ok(mut text) = text_query.get_mut(child) {
                        update_choices(choice.choice, choice.beat, &mut text);
                    }
                }
            }
        }
    }
}

fn update_choices(choice: Choice, beat: i32, text: &mut Text) {
    **text = choice.to_string();
}
