use bevy::prelude::*;

mod controls_popup;
mod element_popup;
mod health_popup;

use controls_popup::ControlsPopupPlugin;
use element_popup::ElementPopupPlugin;
use health_popup::HealthPopupPlugin;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HealthPopupPlugin);
        app.add_plugins(ElementPopupPlugin);
        app.add_plugins(ControlsPopupPlugin);
    }
}
