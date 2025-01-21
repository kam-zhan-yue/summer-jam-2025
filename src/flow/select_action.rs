use bevy::prelude::*;

use crate::{globals::UiAssets, helper::despawn, schedule::GameSet, state::GameFlow};

use super::{Flow, SELECT_ACTION_TIME};

#[derive(Component, Debug)]
struct SelectActionPopup;

pub struct SelectActionPlugin;

impl Plugin for SelectActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameFlow::SelectAction),
            on_enter.in_set(GameSet::Ui),
        );
        app.add_systems(
            OnExit(GameFlow::SelectAction),
            despawn::<SelectActionPopup>.in_set(GameSet::Ui),
        );
    }
}

fn on_enter(mut commands: Commands, ui_assets: Res<UiAssets>, mut flow: ResMut<Flow>) {
    flow.reset(Timer::from_seconds(SELECT_ACTION_TIME, TimerMode::Once));
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            SelectActionPopup,
        ))
        .with_child((
            Text::new("SELECT ACTION"),
            TextFont {
                font: ui_assets.fira_sans_bold.clone(),
                font_size: 72.0,
                ..default()
            },
            TextColor(Color::BLACK),
        ));
}
