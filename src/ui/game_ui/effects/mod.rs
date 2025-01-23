use bevy::prelude::*;

mod element_popup;
mod health_popup;

use element_popup::ElementPopupPlugin;
use health_popup::HealthPopupPlugin;

#[derive(Component, Debug)]
struct EffectsPopup;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_plugins(HealthPopupPlugin);
        app.add_plugins(ElementPopupPlugin);
    }
}

fn setup(mut commands: Commands) {
    println!("Spawn Effects Popup");
    commands.spawn((
        Name::new("EffectsPopup"),
        EffectsPopup,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
    ));
}
