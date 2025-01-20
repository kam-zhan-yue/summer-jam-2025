use bevy::{color::palettes::basic::*, prelude::*};

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn handle_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut colour, mut border_colour) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *colour = PRESSED_BUTTON.into();
                border_colour.0 = RED.into();
            }
            Interaction::Hovered => {
                *colour = HOVERED_BUTTON.into();
                border_colour.0 = Color::WHITE;
            }
            Interaction::None => {
                *colour = NORMAL_BUTTON.into();
                border_colour.0 = Color::BLACK;
            }
        }
    }
}
