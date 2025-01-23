use bevy::prelude::*;

use crate::combo::GameData;
use crate::events::SelectElementEvent;
use crate::helper::despawn;
use crate::schedule::GameSet;
use crate::state::GameState;

use crate::globals::UiAssets;

use super::EffectsPopup;

#[derive(Component, Debug)]
struct PlayerOneElement;

#[derive(Component, Debug)]
struct PlayerTwoElement;

#[derive(Component, Debug)]
struct ElementPopup;

pub struct ElementPopupPlugin;

impl Plugin for ElementPopupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameStart), setup);
        app.add_systems(Update, apply_effects.in_set(GameSet::Ui));
        app.add_systems(
            OnEnter(GameState::GameOver),
            despawn::<ElementPopup>.in_set(GameSet::Ui),
        );
    }
}

fn setup(
    mut commands: Commands,
    query: Query<Entity, With<EffectsPopup>>,
    ui_assets: Res<UiAssets>,
) {
    let Ok(effects_popup) = query.get_single() else {
        return;
    };

    commands
        .entity(effects_popup)
        .with_child((Name::new("Element Popup"), ElementPopup));
}

fn apply_effects(
    mut reader: EventReader<SelectElementEvent>,
    player_one_popup: Query<&Children, With<PlayerOneElement>>,
    player_two_popup: Query<&Children, With<PlayerTwoElement>>,
    game_data: Res<GameData>,
    ui_assets: Res<UiAssets>,
) {
    for _ in reader.read() {
        if let Ok(player_one_children) = player_one_popup.get_single() {}

        if let Ok(player_two_children) = player_two_popup.get_single() {}
    }
}
