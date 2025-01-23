use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum GameSet {
    Flow,
    Rhythm,
    Resolve,
    Ui,
}

pub struct SchedulePlugin;

// TODO: Find a way to create entities between UI and flow
impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                GameSet::Rhythm,
                GameSet::Resolve,
                GameSet::Ui,
                GameSet::Flow,
            )
                .chain(),
        );
    }
}
