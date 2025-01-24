use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

mod controls_popup;
mod element_popup;
mod health_popup;

use controls_popup::ControlsPopupPlugin;
use element_popup::ElementPopupPlugin;
use health_popup::HealthPopupPlugin;

use crate::{
    config::{BGM_VOLUME, START_STATE},
    globals::AudioAssets,
    helper::despawn,
    schedule::GameSet,
    state::GameState,
};

pub struct HudPlugin;

#[derive(Component, Debug)]
struct GameLoop;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HealthPopupPlugin);
        app.add_plugins(ElementPopupPlugin);
        app.add_plugins(ControlsPopupPlugin);
        app.add_systems(OnEnter(START_STATE), start_audio.in_set(GameSet::Flow));
        app.add_systems(
            OnEnter(GameState::GameOver),
            despawn::<GameLoop>.in_set(GameSet::Flow),
        );
    }
}

fn start_audio(mut commands: Commands, audio_assets: Res<AudioAssets>) {
    commands.spawn((
        GameLoop,
        AudioPlayer::new(audio_assets.game_loop.clone()),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new(BGM_VOLUME),
            ..default()
        },
    ));
}
