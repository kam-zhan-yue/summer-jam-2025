use crate::combo::{ChoiceEvent, GameData};
use crate::globals::UiAssets;
use crate::helper::{hide, show};
use crate::rhythm::{Rhythm, BEAT_LIMIT};
use crate::schedule::GameSet;
use crate::state::{GameFlow, GameState};
use crate::types::{Choice, Outcome, Player};
use bevy::prelude::*;

const DISABLED_COLOUR: Color = Color::srgb(0.1, 0.1, 0.1);
const ACTIVE_COLOUR: Color = Color::WHITE;
const LOSS_COLOUR: Color = Color::srgb(0.2, 0.2, 0.2);
const WON_COLOUR: Color = Color::srgb(0.2, 0.8, 0.2);

pub struct ChoicesPlugin;

#[derive(Component, Debug, Default)]
#[require(Visibility)]
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
        app.add_systems(OnEnter(GameFlow::Countdown), on_enter_countdown);
        app.add_systems(OnEnter(GameFlow::Reveal), hide::<ChoicesPopup>);
        app.add_systems(
            OnExit(GameFlow::Reveal),
            (show::<ChoicesPopup>, on_show_choices),
        );
    }
}

fn setup(mut commands: Commands, ui_assets: Res<UiAssets>) {
    // Root Node
    commands
        .spawn((
            Name::new("Choices Popup"),
            ChoicesPopup,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                top: Val::Px(75.0),
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
                &ui_assets,
            );
            spawn_player_choices(
                Name::new("Player Two Choices"),
                parent,
                PlayerTwoChoices,
                JustifyContent::End,
                FlexDirection::RowReverse,
                &ui_assets,
            );
        });
}

fn spawn_player_choices(
    name: impl Component,
    parent: &mut ChildBuilder,
    player: impl Component,
    justify_content: JustifyContent,
    flex_direction: FlexDirection,
    ui_assets: &Res<UiAssets>,
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
            for _ in 0..BEAT_LIMIT {
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
                        BackgroundColor(DISABLED_COLOUR),
                    ))
                    .with_child((
                        ImageNode::new(ui_assets.empty.clone()),
                        Node {
                            width: Val::Px(30.0),
                            height: Val::Px(30.0),
                            ..default()
                        },
                        Visibility::Hidden,
                    ));
            }
        });
}

fn on_enter_countdown(
    choices: Query<&Children, Or<(With<PlayerOneChoices>, With<PlayerTwoChoices>)>>,
    mut choice_popup_items: Query<&mut BackgroundColor, With<ChoicePopupItem>>,
    rhythm: Res<Rhythm>,
) {
    // Set the current beat's choices to an active colour
    for children in choices.iter() {
        // Get the current child according to the beat
        let &child = children.get(rhythm.beat as usize).unwrap();
        // Get the children of that popup item
        let mut popup_item = choice_popup_items.get_mut(child).unwrap();
        *popup_item = ACTIVE_COLOUR.into();
    }
}

fn read_choices(
    mut choice_event_reader: EventReader<ChoiceEvent>,
    player_one_choices: Query<&Children, With<PlayerOneChoices>>,
    player_two_choices: Query<&Children, With<PlayerTwoChoices>>,
    choice_popup_items: Query<&Children, With<ChoicePopupItem>>,
    mut image_query: Query<(&mut ImageNode, &mut Visibility)>,
    ui_assets: Res<UiAssets>,
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
                    if let Ok(mut image) = image_query.get_mut(child) {
                        // Set to visible and unknown icon
                        *image.1 = Visibility::Inherited;
                        image.0.image = ui_assets.unknown.clone();
                    }
                }
            }
        }
    }
}

fn on_show_choices(
    player_one_choices: Query<&Children, With<PlayerOneChoices>>,
    player_two_choices: Query<&Children, With<PlayerTwoChoices>>,
    mut choice_popup_items: Query<(&mut BackgroundColor, &Children), With<ChoicePopupItem>>,
    mut image_query: Query<&mut ImageNode>,
    rhythm: Res<Rhythm>,
    game_data: Res<GameData>,
) {
    // Here, we want to set the previous result
    // let Ok(player_one_children) = player_one_choices.get_single() else {
    //     return;
    // };
    // let Ok(player_two_children) = player_two_choices.get_single() else {
    //     return;
    // };

    // let prev_beat = rhythm.beat - 1;

    // let result = game_data.get_result(prev_beat);
    // let player_one_choice = game_data.get_choice(Player::One, prev_beat);
    // let player_two_choice = game_data.get_choice(Player::Two, prev_beat);

    // let &player_two_pointer = player_one_children.get(prev_beat as usize).unwrap();
    // let mut player_two_item = choice_popup_items.get_mut(player_two_pointer).unwrap();

    // let &player_one_pointer = player_one_children.get(prev_beat as usize).unwrap();
    // let mut player_one_item = choice_popup_items.get_mut(player_one_pointer).unwrap();
    // // Change the background colour according to if they won or lost
    // match result.outcome {
    //     Outcome::Draw => {
    //         // Set both to inactive
    //         *player_one_item.0 = LOSS_COLOUR.into();
    //         *player_two_item.0 = LOSS_COLOUR.into();
    //     }
    //     Outcome::PlayerOne => {
    //         *player_one_item.0 = WON_COLOUR.into();
    //         *player_two_item.0 = LOSS_COLOUR.into();
    //     }
    //     Outcome::PlayerTwo => {
    //         *player_one_item.0 = LOSS_COLOUR.into();
    //         *player_two_item.0 = WON_COLOUR.into();
    //     }
    // }
}
