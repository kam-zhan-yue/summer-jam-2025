use crate::helper::despawn;
use crate::rhythm::Rhythm;
use crate::schedule::GameSet;
use crate::state::GameFlow;
use bevy::prelude::*;

const TITLE_TIME: f32 = 2.;

#[derive(Component, Debug)]
struct Title {
    timer: Timer,
}

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameFlow::Title), spawn_title);
        app.add_systems(
            Update,
            update_title
                .in_set(GameSet::Ui)
                .run_if(in_state(GameFlow::Title)),
        );
        app.add_systems(OnExit(GameFlow::Title), despawn::<Title>);
    }
}

fn spawn_title(mut commands: Commands, rhythm: Res<Rhythm>, asset_server: ResMut<AssetServer>) {
    // Root Node
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            Title {
                timer: Timer::from_seconds(TITLE_TIME, TimerMode::Once),
            },
        ))
        .with_child((
            Text::new(get_title(rhythm.beat)),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 100.0,
                ..default()
            },
            TextColor(Color::BLACK),
        ));
}

fn get_title(beat: i32) -> String {
    match beat {
        0 => "TOOLS".to_string(),
        1 => "LOCATION".to_string(),
        _ => "NONE".to_string(),
    }
}

fn update_title(
    time: Res<Time>,
    mut query: Query<&mut Title>,
    mut game_flow: ResMut<NextState<GameFlow>>,
) {
    let Ok(mut title) = query.get_single_mut() else {
        return;
    };
    title.timer.tick(time.delta());
    if title.timer.just_finished() {
        game_flow.set(GameFlow::Countdown);
    }
}
