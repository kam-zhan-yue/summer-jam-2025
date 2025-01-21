use crate::combo::GameData;
use crate::helper::despawn;
use crate::rhythm::Rhythm;
use crate::schedule::GameSet;
use crate::state::{GameFlow, GameState};
use bevy::prelude::*;

const END_TURN_TIME: f32 = 3.0;

#[derive(Component, Debug)]
struct EndTurnPopup {
    timer: Timer,
}

pub struct EndTurnPlugin;

impl Plugin for EndTurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameFlow::EndTurn), on_enter_end_turn);
        app.add_systems(
            Update,
            handle_timers
                .in_set(GameSet::Ui)
                .run_if(in_state(GameFlow::EndTurn)),
        );
        app.add_systems(OnExit(GameFlow::EndTurn), despawn::<EndTurnPopup>);
    }
}

fn on_enter_end_turn(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    mut game_flow: ResMut<NextState<GameFlow>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((
            EndTurnPopup {
                timer: Timer::from_seconds(END_TURN_TIME, TimerMode::Once),
            },
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_child((
            Text::new(get_text(&mut game_data)),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 64.0,
                ..default()
            },
            TextColor(Color::BLACK),
        ));
}

fn get_text(game_data: &mut GameData) -> String {
    "End Turn".to_string()
}

fn handle_timers(
    mut query: Query<&mut EndTurnPopup>,
    mut game_flow: ResMut<NextState<GameFlow>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut rhythm: ResMut<Rhythm>,
    game_data: Res<GameData>,
    time: Res<Time>,
) {
    let Ok(mut end_turn_popup) = query.get_single_mut() else {
        return;
    };

    end_turn_popup.timer.tick(time.delta());
    if end_turn_popup.timer.just_finished() {
        rhythm.reset();
        if game_data.can_end_game() {
            game_state.set(GameState::GameOver);
        } else {
            game_flow.set(GameFlow::Title);
        }
    }
}
