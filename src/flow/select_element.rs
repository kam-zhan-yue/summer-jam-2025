use bevy::prelude::*;

use crate::{
    combo::GameData,
    globals::UiAssets,
    helper::despawn,
    schedule::GameSet,
    state::{GameFlow, UiFlow},
};

use super::{
    countdown::Countdown, Flow, COUNTDOWN_TIME, REVEAL_TIME, SELECT_ELEMENT_TIME, TITLE_TIME,
};

#[derive(Component, Debug)]
struct SelectElementPopup;

#[derive(Component, Debug)]
struct SelectElementTitle;

pub struct SelectElementPlugin;

impl Plugin for SelectElementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameFlow::SelectElement),
            on_enter.in_set(GameSet::Ui),
        );
        app.add_systems(
            Update,
            (handle_countdown, handle_input)
                .in_set(GameSet::Ui)
                .run_if(in_state(GameFlow::SelectElement)),
        );
        // Hide the title when out of the Title Flow
        app.add_systems(
            OnExit(UiFlow::Title),
            despawn::<SelectElementTitle>
                .in_set(GameSet::Ui)
                .run_if(in_state(GameFlow::SelectElement)),
        );
        app.add_systems(
            OnExit(GameFlow::SelectElement),
            despawn::<SelectElementPopup>.in_set(GameSet::Ui),
        );
    }
}

fn on_enter(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    mut countdown: ResMut<Countdown>,
    mut next_ui: ResMut<NextState<UiFlow>>,
) {
    countdown.reset(Timer::from_seconds(TITLE_TIME, TimerMode::Once));
    next_ui.set(UiFlow::Title);
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            SelectElementPopup,
        ))
        .with_child((
            SelectElementTitle,
            Text::new("SELECT ELEMENT"),
            TextFont {
                font: ui_assets.fira_sans_bold.clone(),
                font_size: 72.0,
                ..default()
            },
            TextColor(Color::BLACK),
        ));
}

fn handle_countdown(
    mut countdown: ResMut<Countdown>,
    current_ui_flow: Res<State<UiFlow>>,
    mut next_ui_flow: ResMut<NextState<UiFlow>>,
    mut flow: ResMut<Flow>,
    time: Res<Time>,
) {
    countdown.tick(time.delta());
    if countdown.timer.just_finished() {
        match current_ui_flow.get() {
            // Go to the countdown after the title
            UiFlow::Title => {
                countdown.reset(Timer::from_seconds(COUNTDOWN_TIME, TimerMode::Once));
                next_ui_flow.set(UiFlow::Countdown)
            }
            // Go to the reveal after the countdown
            UiFlow::Countdown => {
                countdown.reset(Timer::from_seconds(REVEAL_TIME, TimerMode::Once));
                next_ui_flow.set(UiFlow::Reveal);
            }
            // Go to the next stage after the reveal
            UiFlow::Reveal => flow.reset(Timer::from_seconds(SELECT_ELEMENT_TIME, TimerMode::Once)),
        }
    }
}

fn handle_input(current_ui_flow: Res<State<UiFlow>>, mut game_data: ResMut<GameData>) {
    if *current_ui_flow.get() != UiFlow::Countdown {
        return;
    }
}
