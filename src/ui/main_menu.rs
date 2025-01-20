use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct MainMenu;

#[derive(Component, Debug)]
pub struct PlayButton;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_main_menu);
    }
}

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the Root Node
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                row_gap: Val::Px(8.),
                column_gap: Val::Px(8.),
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            // Title
            parent
                .spawn(
                    (Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        ..default()
                    }),
                )
                .with_children(|parent| {
                    // Text
                    parent.spawn((
                        Text::new("Text Example"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 25.0,
                            ..default()
                        },
                        Label,
                    ));
                });
        });
}
