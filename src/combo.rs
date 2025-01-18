use crate::schedule::GameSet;
use bevy::prelude::*;

pub struct ComboPlugin;

impl Plugin for ComboPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_input, update_combo).chain().in_set(GameSet::Combo),
        );
        app.add_systems(Update, resolve_combo.in_set(GameSet::Resolve));
    }
}

fn handle_input() {}

fn update_combo() {}

fn resolve_combo() {}
