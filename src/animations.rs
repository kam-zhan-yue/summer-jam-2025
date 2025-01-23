use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{
    lens::{TransformScaleLens, UiBackgroundColorLens, UiPositionLens},
    Tween,
};

use crate::{
    camera::{SCREEN_X, SCREEN_Y},
    config::{
        ANIM_FADE_IN, ANIM_FADE_OUT, ANIM_SCALE_DOWN, ANIM_SCALE_UP, ANIM_SCROLL_LEFT,
        ANIM_SCROLL_RIGHT,
    },
};

const OFFSET: f32 = 500.;

pub fn move_in_tween(width: &f32, height: &f32) -> Tween<Node> {
    return Tween::new(
        EaseFunction::QuarticOut,
        Duration::from_millis(ANIM_SCROLL_LEFT),
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
        Duration::from_millis(ANIM_SCROLL_RIGHT),
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

pub fn fade_in() -> Tween<BackgroundColor> {
    Tween::new(
        EaseFunction::QuarticOut,
        Duration::from_millis(ANIM_FADE_IN),
        UiBackgroundColorLens {
            start: Color::srgba(0.0, 0.0, 0.0, 0.0),
            end: Color::srgba(0.0, 0.0, 0.0, 0.9),
        },
    )
}

pub fn fade_out() -> Tween<BackgroundColor> {
    Tween::new(
        EaseFunction::QuarticOut,
        Duration::from_millis(ANIM_FADE_OUT),
        UiBackgroundColorLens {
            start: Color::srgba(0.0, 0.0, 0.0, 0.8),
            end: Color::srgba(0.0, 0.0, 0.0, 0.0),
        },
    )
}

pub fn scale_up() -> Tween<Transform> {
    Tween::new(
        EaseFunction::QuarticOut,
        Duration::from_millis(ANIM_SCALE_UP),
        TransformScaleLens {
            start: Vec3::ZERO,
            end: Vec3::ONE,
        },
    )
}

pub fn scale_down() -> Tween<Transform> {
    Tween::new(
        EaseFunction::QuarticOut,
        Duration::from_millis(ANIM_SCALE_DOWN),
        TransformScaleLens {
            start: Vec3::ONE,
            end: Vec3::ZERO,
        },
    )
}
