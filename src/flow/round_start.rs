use bevy::prelude::*;

use crate::helper::despawn;
use crate::schedule::GameSet;
use crate::{globals::UiAssets, state::GameState};

use super::{Flow, ROUND_START_TIME};

pub struct RoundStartPlugin;

#[derive(Component, Debug)]
struct RoundStartPopup;

impl Plugin for RoundStartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameStart), on_enter.in_set(GameSet::Ui));
        app.add_systems(
            OnExit(GameState::GameStart),
            despawn::<RoundStartPopup>.in_set(GameSet::Ui),
        );
    }
}

fn on_enter(mut commands: Commands, ui_assets: Res<UiAssets>, mut flow: ResMut<Flow>) {
    flow.reset(Timer::from_seconds(ROUND_START_TIME, TimerMode::Once));
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            RoundStartPopup,
        ))
        .with_child((
            Text::new("START"),
            TextFont {
                font: ui_assets.fira_sans_bold.clone(),
                font_size: 72.0,
                ..default()
            },
            TextColor(Color::BLACK),
        ));
}
