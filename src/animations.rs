use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{
    lens::{TransformPositionLens, TransformScaleLens, UiBackgroundColorLens, UiPositionLens},
    Sequence, Tween,
};

use crate::config::*;

pub fn won_sequence() -> Sequence<BackgroundColor> {
    let fade_in = Tween::new(
        EaseFunction::QuarticInOut,
        Duration::from_millis(ANIM_FADE_IN_COLOUR),
        UiBackgroundColorLens {
            start: Color::WHITE,
            end: WON_COLOUR_SOLID,
        },
    );
    let fade_out = Tween::new(
        EaseFunction::QuarticOut,
        Duration::from_millis(ANIM_FADE_OUT_COLOUR),
        UiBackgroundColorLens {
            start: WON_COLOUR_SOLID,
            end: WON_COLOUR_TRANSPARENT,
        },
    );
    fade_in.then(fade_out)
}

pub fn loss_sequence() -> Sequence<BackgroundColor> {
    let fade_in = Tween::new(
        EaseFunction::QuarticInOut,
        Duration::from_millis(ANIM_FADE_IN_COLOUR),
        UiBackgroundColorLens {
            start: Color::WHITE,
            end: LOSS_COLOUR_SOLID,
        },
    );
    let fade_out = Tween::new(
        EaseFunction::QuarticOut,
        Duration::from_millis(ANIM_FADE_OUT_COLOUR),
        UiBackgroundColorLens {
            start: LOSS_COLOUR_SOLID,
            end: LOSS_COLOUR_TRANSPARENT,
        },
    );
    fade_in.then(fade_out)
}

const OFFSET: f32 = 500.;

pub fn move_in_tween(
    screen_width: &f32,
    screen_height: &f32,
    width: &f32,
    height: &f32,
) -> Tween<Node> {
    return Tween::new(
        EaseFunction::QuarticOut,
        Duration::from_millis(ANIM_SCROLL_LEFT),
        UiPositionLens {
            start: UiRect {
                left: Val::Px(-screen_width as f32 - OFFSET - width),
                top: Val::Px(screen_height / 2. - height / 2.),
                right: Val::Auto,
                bottom: Val::Auto,
            },
            end: UiRect {
                left: Val::Px(screen_width / 2. - width / 2.),
                top: Val::Px(screen_height / 2. - height / 2.),
                right: Val::Auto,
                bottom: Val::Auto,
            },
        },
    );
}
pub fn move_out_tween(
    screen_width: &f32,
    screen_height: &f32,
    width: &f32,
    height: &f32,
) -> Tween<Node> {
    Tween::new(
        EaseFunction::QuarticIn,
        Duration::from_millis(ANIM_SCROLL_RIGHT),
        UiPositionLens {
            start: UiRect {
                left: Val::Px(screen_width / 2. - width / 2.),
                top: Val::Px(screen_height / 2. - height / 2.),
                right: Val::Auto,
                bottom: Val::Auto,
            },
            end: UiRect {
                left: Val::Px(screen_width + OFFSET + width),
                top: Val::Px(screen_height / 2. - height / 2.),
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
            start: TRANSPARENT,
            end: DARK,
        },
    )
}

pub fn fade_out() -> Tween<BackgroundColor> {
    Tween::new(
        EaseFunction::QuarticOut,
        Duration::from_millis(ANIM_FADE_OUT),
        UiBackgroundColorLens {
            start: DARK,
            end: TRANSPARENT,
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

pub fn shake_player_sequence(original_pos: &Vec3, left: bool) -> Sequence<Transform> {
    let move_right = Tween::new(
        EaseFunction::Linear,
        Duration::from_millis(ANIM_SHAKE),
        TransformPositionLens {
            start: Vec3::new(original_pos.x, original_pos.y, original_pos.z),
            end: Vec3::new(original_pos.x + SHAKE_X, original_pos.y, original_pos.z),
        },
    );

    let return_from_right = Tween::new(
        EaseFunction::Linear,
        Duration::from_millis(ANIM_SHAKE),
        TransformPositionLens {
            start: Vec3::new(original_pos.x + SHAKE_X, original_pos.y, original_pos.z),
            end: Vec3::new(original_pos.x, original_pos.y, original_pos.z),
        },
    );

    let move_left = Tween::new(
        EaseFunction::Linear,
        Duration::from_millis(ANIM_SHAKE),
        TransformPositionLens {
            start: Vec3::new(original_pos.x, original_pos.y, original_pos.z),
            end: Vec3::new(original_pos.x, original_pos.y, original_pos.z - SHAKE_X),
        },
    );

    let return_from_left = Tween::new(
        EaseFunction::Linear,
        Duration::from_millis(ANIM_SHAKE),
        TransformPositionLens {
            start: Vec3::new(original_pos.x - SHAKE_X, original_pos.y, original_pos.z),
            end: Vec3::new(original_pos.x, original_pos.y, original_pos.z),
        },
    );
    if left {
        move_left.then(return_from_left.then(move_right).then(return_from_right))
    } else {
        move_right.then(return_from_right.then(move_left).then(return_from_left))
    }
}
