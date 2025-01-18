use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum GameSet {
    Rhythm,
    Combo,
    Resolve,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (GameSet::Rhythm, GameSet::Combo, GameSet::Resolve).chain(),
        );
    }
}
