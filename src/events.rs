use bevy::prelude::*;

use crate::types::{Choice, Player};

#[derive(Event, Debug, Default)]
pub struct ApplyEffectsEvent;

#[derive(Event, Debug, Default)]
pub struct SelectElementEvent {
    player: Player,
    element: Choice,
}

impl SelectElementEvent {
    pub fn new(player: Player, element: Choice) -> Self {
        Self { player, element }
    }
}

#[derive(Event, Debug, Default)]
pub struct SelectActionEvent {
    player: Player,
    action: Choice,
}

impl SelectActionEvent {
    pub fn new(player: Player, action: Choice) -> Self {
        Self { player, action }
    }
}

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ApplyEffectsEvent>();
        app.add_event::<SelectElementEvent>();
        app.add_event::<SelectActionEvent>();
    }
}
