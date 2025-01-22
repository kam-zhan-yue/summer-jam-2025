use bevy::prelude::*;

#[derive(Event, Debug, Default)]
pub struct ApplyEffectsEvent;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ApplyEffectsEvent>();
    }
}
