use bevy::{color::palettes::basic::*, prelude::*};
use rand::seq::SliceRandom;
use rand::thread_rng; // 0.7.2

use crate::config::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};

pub fn get_random<T>(vec: &Vec<T>) -> &T {
    let mut rng = thread_rng();
    vec.choose(&mut rng).unwrap()
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn show<T: Component>(mut query: Query<&mut Visibility, With<T>>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

pub fn hide<T: Component>(mut query: Query<&mut Visibility, With<T>>) {
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
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
