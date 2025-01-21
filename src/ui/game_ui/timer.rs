use crate::flow::Flow;
use crate::globals::UiAssets;
use crate::helper::{hide, show};
use crate::schedule::GameSet;
use crate::state::{GameFlow, GameState};
use bevy::prelude::*;

pub struct TimerPlugin;

#[derive(Component, Debug, Default)]
struct TimerPopup;

#[derive(Component, Debug, Default)]
struct Timer;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup);
        app.add_systems(
            Update,
            update_timer
                .in_set(GameSet::Ui)
                .run_if(in_state(GameState::Game)),
        );
        app.add_systems(OnEnter(GameFlow::Countdown), show::<Timer>);
        app.add_systems(OnExit(GameFlow::Countdown), hide::<Timer>);
    }
}

fn setup(mut commands: Commands, ui_assets: Res<UiAssets>) {
    // Root Node
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            TimerPopup,
        ))
        .with_child((
            Timer,
            Text::new("3.0"),
            TextFont {
                font: ui_assets.fira_sans_bold.clone(),
                font_size: 30.0,
                ..default()
            },
            TextColor(Color::BLACK),
        ));
}

fn update_timer(mut query: Query<&mut Text, With<Timer>>, flow: Res<Flow>) {
    let Ok(mut timer) = query.get_single_mut() else {
        return;
    };
    **timer = format!("{:.1}", flow.timer.remaining_secs());
}
