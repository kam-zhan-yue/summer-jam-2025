use bevy::ui::{UiRect, Val};
use bevy_color::Color;

// Max Health of the player
pub const MAX_HEALTH: i32 = 5;

// ====== FONTS ======
pub const SIZE_XXL: f32 = 128.0;
pub const SIZE_XL: f32 = 64.0;
pub const SIZE_L: f32 = 48.0;
pub const SIZE_M: f32 = 32.0;
pub const SIZE_S: f32 = 24.0;
pub const SIZE_XS: f32 = 16.0;

// ====== BUTTONS ======
pub const BUTTON_WIDTH: Val = Val::Px(200.0);
pub const BUTTON_HEIGHT: Val = Val::Px(50.0);
pub const BUTTON_BORDER: UiRect = UiRect::all(Val::Px(2.0));
pub const BORDER_RADIUS: Val = Val::Px(5.0);

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// ====== ANIMATIONS ======
pub const ANIM_SCROLL_LEFT: u64 = 1200;
pub const ANIM_SCROLL_RIGHT: u64 = 1200;
pub const ANIM_FADE_IN: u64 = 700;
pub const ANIM_FADE_OUT: u64 = 2000;
pub const ANIM_SCALE_UP: u64 = 1000;
pub const ANIM_SCALE_DOWN: u64 = 100;
pub const TRANSPARENT: Color = Color::srgba(0.0, 0.0, 0.0, 0.0);

// Time Spent on Initial Start Round Title
pub const ROUND_START_TIME: f32 = 3.0;
// Time Spent Displaying Transition Titles
pub const TITLE_TIME: f32 = 1.0;
// Time Allowed to Choose an Element / Action
pub const COUNTDOWN_TIME: f32 = 2.0;
// Time Spent to Reveal Elements / Actions
pub const REVEAL_TIME: f32 = 2.0;
