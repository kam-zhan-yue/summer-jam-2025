use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{
    lens::{
        SpriteColorLens, TransformPositionLens, TransformScaleLens, UiBackgroundColorLens,
        UiPositionLens,
    },
    Sequence, Tween,
};

use crate::{
    camera::{SCREEN_X, SCREEN_Y},
    config::*,
};

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

pub fn move_in_tween(width: &f32, height: &f32) -> Tween<Node> {
    return Tween::new(
        EaseFunction::QuarticOut,
        Duration::from_millis(ANIM_SCROLL_LEFT),
        UiPositionLens {
            start: UiRect {
                left: Val::Px(-SCREEN_X as f32 - OFFSET - width),
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

pub fn shake_player_sequence(transform: &Transform, left: bool) -> Sequence<Transform> {
    let move_right = Tween::new(
        EaseFunction::Linear,
        Duration::from_millis(ANIM_SHAKE),
        TransformPositionLens {
            start: Vec3::new(
                transform.translation.x,
                transform.translation.y,
                transform.translation.z,
            ),
            end: Vec3::new(
                transform.translation.x + SHAKE_X,
                transform.translation.y,
                transform.translation.z,
            ),
        },
    );

    let return_from_right = Tween::new(
        EaseFunction::Linear,
        Duration::from_millis(ANIM_SHAKE),
        TransformPositionLens {
            start: Vec3::new(
                transform.translation.x + SHAKE_X,
                transform.translation.y,
                transform.translation.z,
            ),
            end: Vec3::new(
                transform.translation.x,
                transform.translation.y,
                transform.translation.z,
            ),
        },
    );

    let move_left = Tween::new(
        EaseFunction::Linear,
        Duration::from_millis(ANIM_SHAKE),
        TransformPositionLens {
            start: Vec3::new(
                transform.translation.x,
                transform.translation.y,
                transform.translation.z,
            ),
            end: Vec3::new(
                transform.translation.x,
                transform.translation.y,
                transform.translation.z - SHAKE_X,
            ),
        },
    );

    let return_from_left = Tween::new(
        EaseFunction::Linear,
        Duration::from_millis(ANIM_SHAKE),
        TransformPositionLens {
            start: Vec3::new(
                transform.translation.x - SHAKE_X,
                transform.translation.y,
                transform.translation.z,
            ),
            end: Vec3::new(
                transform.translation.x,
                transform.translation.y,
                transform.translation.z,
            ),
        },
    );
    if left {
        move_left.then(return_from_left.then(move_right).then(return_from_right))
    } else {
        move_right.then(return_from_right.then(move_left).then(return_from_left))
    }
}
