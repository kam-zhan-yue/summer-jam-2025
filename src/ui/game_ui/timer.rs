use crate::helper::{hide, show};
use crate::rhythm::Rhythm;
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
        app.add_systems(OnEnter(GameFlow::Reveal), hide::<Timer>);
        app.add_systems(OnExit(GameFlow::Reveal), show::<Timer>);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::BLACK),
        ));
}

fn update_timer(mut query: Query<&mut Text, With<Timer>>, rhythm: Res<Rhythm>) {
    let Ok(mut timer) = query.get_single_mut() else {
        return;
    };
    **timer = format!("{:.1}", rhythm.timer.remaining_secs());
}
