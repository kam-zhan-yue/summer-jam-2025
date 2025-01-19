use bevy::prelude::*;

pub const MAIN_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style
}

pub const TITLE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent
}
