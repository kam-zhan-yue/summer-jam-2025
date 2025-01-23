use bevy::prelude::*;

use crate::combo::GameData;
use crate::config::{SIZE_M, SIZE_S, START_STATE};
use crate::events::SelectElementEvent;
use crate::helper::{despawn, hide, show};
use crate::schedule::GameSet;
use crate::state::{GameState, UiState};

use crate::globals::UiAssets;
use crate::types::Player;

#[derive(Component, Debug)]
struct PlayerOneElement;

#[derive(Component, Debug)]
struct PlayerTwoElement;

#[derive(Component, Debug)]
struct ElementPopup;

pub struct ElementPopupPlugin;

impl Plugin for ElementPopupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(START_STATE), setup);
        app.add_systems(Update, apply_effects.in_set(GameSet::Ui));
        app.add_systems(
            OnEnter(GameState::GameOver),
            despawn::<ElementPopup>.in_set(GameSet::Ui),
        );
        app.add_systems(OnEnter(UiState::Title), hide::<ElementPopup>);
        app.add_systems(OnExit(UiState::Title), show::<ElementPopup>);
    }
}

fn setup(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands
        .spawn((
            Name::new("Element Popup"),
            ElementPopup,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(75.0), Val::Px(10.0)),
                align_items: AlignItems::FlexStart,
                ..default()
            },
        ))
        .with_children(|parent| {
            spawn_element_popup(
                Name::new("Player One Element"),
                parent,
                PlayerOneElement,
                JustifyContent::Start,
                &ui_assets,
            );
            spawn_element_popup(
                Name::new("Player Two Element"),
                parent,
                PlayerTwoElement,
                JustifyContent::End,
                &ui_assets,
            );
        });
}

fn spawn_element_popup(
    name: Name,
    parent: &mut ChildBuilder,
    player: impl Component,
    justify_content: JustifyContent,
    ui_assets: &Res<UiAssets>,
) {
    parent
        .spawn((
            name,
            player,
            Node {
                width: Val::Percent(50.0),
                justify_content,
                align_items: AlignItems::Center,
                ..default()
            },
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageNode::new(ui_assets.element_fire.clone()),
                Node {
                    width: Val::Px(50.0),
                    height: Val::Px(50.0),
                    ..default()
                },
            ));
            parent.spawn((
                Text::new(" attacks do double damage!"),
                TextFont {
                    font: ui_assets.ms_pain.clone(),
                    font_size: SIZE_S,
                    ..default()
                },
                TextColor::BLACK,
            ));
        });
}

fn apply_effects(
    mut reader: EventReader<SelectElementEvent>,
    mut player_one_popup: Query<(&mut Visibility, &Children), With<PlayerOneElement>>,
    mut player_two_popup: Query<
        (&mut Visibility, &Children),
        (With<PlayerTwoElement>, Without<PlayerOneElement>),
    >,
    mut image_query: Query<&mut ImageNode>,
    mut text_query: Query<&mut Text>,
    ui_assets: Res<UiAssets>,
) {
    let Ok(mut player_one) = player_one_popup.get_single_mut() else {
        return;
    };
    let Ok(mut player_two) = player_two_popup.get_single_mut() else {
        return;
    };
    for event in reader.read() {
        match event.player {
            Player::One => {
                *player_one.0 = Visibility::Visible;
                for &child in player_one.1 {
                    if let Ok(mut text) = text_query.get_mut(child) {
                        **text = "Test".to_string();
                    }
                    if let Ok(mut image) = image_query.get_mut(child) {
                        *image = ImageNode::new(ui_assets.tool_hand.clone());
                    }
                }
            }
            Player::Two => {
                for &child in player_two.1 {
                    *player_two.0 = Visibility::Visible;
                    if let Ok(mut text) = text_query.get_mut(child) {
                        **text = "Test".to_string();
                    }
                    if let Ok(mut image) = image_query.get_mut(child) {
                        *image = ImageNode::new(ui_assets.tool_hand.clone());
                    }
                }
            }
        }
    }
}
