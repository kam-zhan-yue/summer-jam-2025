use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::UiPositionLens, Tween};

use crate::camera::{SCREEN_X, SCREEN_Y};

const OFFSET: f32 = 500.;

pub fn move_in_tween(width: &f32, height: &f32) -> Tween<Node> {
    return Tween::new(
        EaseFunction::QuarticOut,
        Duration::from_millis(1200),
        UiPositionLens {
            start: UiRect {
                left: Val::Px(-SCREEN_X - OFFSET - width),
                top: Val::Px(SCREEN_Y / 2. - height / 2.),
                right: Val::Auto,
                bottom: Val::Auto,
            },
            end: UiRect {
                left: Val::Px(SCREEN_X / 2. - width / 2.),
                top: Val::Px(SCREEN_Y / 2. - height / 2.),
                right: Val::Auto,
                bottom: Val::Auto,
            },
        },
    );
}
pub fn move_out_tween(width: &f32, height: &f32) -> Tween<Node> {
    Tween::new(
        EaseFunction::QuarticIn,
        Duration::from_millis(1200),
        UiPositionLens {
            start: UiRect {
                left: Val::Px(SCREEN_X / 2. - width / 2.),
                top: Val::Px(SCREEN_Y / 2. - height / 2.),
                right: Val::Auto,
                bottom: Val::Auto,
            },
            end: UiRect {
                left: Val::Px(SCREEN_X + OFFSET + width),
                top: Val::Px(SCREEN_Y / 2. - height / 2.),
                right: Val::Auto,
                bottom: Val::Auto,
            },
        },
    )
}
