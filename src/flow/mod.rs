use bevy::prelude::*;
use combo_breaker::ComboBreakerPlugin;
use resolve_action::ResolveActionPlugin;
use round_over::RoundOverPlugin;
use round_start::RoundStartPlugin;
use select_action::SelectActionPlugin;
use select_element::SelectElementPlugin;

mod combo_breaker;
mod resolve_action;
mod round_over;
mod round_start;
mod select_action;
mod select_element;

pub struct FlowPlugin;

impl Plugin for FlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RoundStartPlugin);
        app.add_plugins(SelectElementPlugin);
        app.add_plugins(SelectActionPlugin);
        app.add_plugins(ResolveActionPlugin);
        app.add_plugins(ComboBreakerPlugin);
        app.add_plugins(RoundOverPlugin);
    }
}
