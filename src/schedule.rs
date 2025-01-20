use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum GameSet {
    Rhythm,
    Resolve,
    Ui,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (GameSet::Rhythm, GameSet::Resolve, GameSet::Ui).chain(),
        );
    }
}
