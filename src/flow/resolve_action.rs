use bevy::prelude::*;

use crate::{schedule::GameSet, state::GameFlow};

pub struct ResolveActionPlugin;

impl Plugin for ResolveActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameFlow::ResolveAction),
            on_enter.in_set(GameSet::Ui),
        );
    }
}

fn on_enter() {}
