use std::time::Duration;

use bevy::prelude::*;

use crate::{
    globals::UiAssets,
    helper::{despawn, hide, show},
    schedule::GameSet,
    state::{GameState, UiState},
};

#[derive(Resource)]
pub struct Countdown {
    pub timer: Timer,
}

impl Countdown {
    pub fn reset(&mut self, timer: Timer) {
        self.timer = timer;
    }

    pub fn tick(&mut self, duration: Duration) {
        self.timer.tick(duration);
    }
}

#[derive(Component, Debug, Default)]
struct CountdownPopup;

#[derive(Component, Debug, Default)]
struct CountdownText;

impl Default for Countdown {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.0, TimerMode::Once),
        }
    }
}

pub struct CountdownPlugin;

impl Plugin for CountdownPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Countdown>();
        app.add_systems(OnEnter(GameState::GameStart), setup);

        // Showing, Updating, and Hiding the CountdownPopup
        app.add_systems(
            OnEnter(UiState::Countdown),
            show::<CountdownPopup>.in_set(GameSet::Ui),
        );
        app.add_systems(
            Update,
            update_timer
                .in_set(GameSet::Ui)
                .run_if(in_state(UiState::Countdown)),
        );
        app.add_systems(
            OnExit(UiState::Countdown),
            hide::<CountdownPopup>.in_set(GameSet::Ui),
        );

        app.add_systems(
            OnEnter(GameState::GameOver),
            despawn::<CountdownPopup>.in_set(GameSet::Ui),
        );
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
            CountdownPopup,
            Visibility::Hidden,
        ))
        .with_child((
            CountdownText,
            Text::new("3.0"),
            TextFont {
                font: ui_assets.fira_sans_bold.clone(),
                font_size: 30.0,
                ..default()
            },
            TextColor(Color::BLACK),
        ));
}

fn update_timer(mut query: Query<&mut Text, With<CountdownText>>, countdown: Res<Countdown>) {
    let Ok(mut timer) = query.get_single_mut() else {
        return;
    };
    **timer = format!("{:.1}", countdown.timer.remaining_secs());
}
