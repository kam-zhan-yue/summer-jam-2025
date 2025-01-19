use crate::styles::*;
use bevy::prelude::*;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the Root Node
    commands
        .spawn((
            Node {
                style: MAIN_MENU_STYLE;
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(
                (Node {
                    style: TITLE_STYLE,
                    ..default()
                }),
            )
        });
}
