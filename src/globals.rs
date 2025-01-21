use bevy::prelude::*;

use crate::types::{Choice, Element, Action};

pub struct GlobalPlugin;

impl Plugin for GlobalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiAssetsPlugin);
    }
}

#[derive(Resource, Debug, Default)]
pub struct UiAssets {
    pub heart_broken: Handle<Image>,
    pub heart_full: Handle<Image>,
    pub empty: Handle<Image>,
    pub unknown: Handle<Image>,
    pub element_fire: Handle<Image>,
    pub element_grass: Handle<Image>,
    pub element_water: Handle<Image>,
    pub tool_hand: Handle<Image>,
    pub tool_toilet: Handle<Image>,
    pub tool_underwear: Handle<Image>,
    pub fira_sans_bold: Handle<Font>,
}

impl UiAssets {
    pub fn get_icon(&self, choice: Choice) -> Handle<Image> {
        match choice {
            Choice::None => self.unknown.clone(),
            Choice::Action(Action::Hand) => self.tool_hand.clone(),
            Choice::Action(Action::Toilet) => self.tool_toilet.clone(),
            Choice::Action(Action::Underwear) => self.tool_underwear.clone(),
            Choice::Element(Element::Fire) => self.element_fire.clone(),
            Choice::Element(Element::Water) => self.element_water.clone(),
            Choice::Element(Element::Grass) => self.element_grass.clone(),
        }
    }
}

#[derive(Component, Debug)]
pub struct UiAssetsPlugin;

impl Plugin for UiAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiAssets>();
        app.add_systems(Startup, setup);
    }
}

fn setup(asset_server: Res<AssetServer>, mut ui_assets: ResMut<UiAssets>) {
    ui_assets.empty = asset_server.load("ui/empty.png");
    ui_assets.unknown = asset_server.load("ui/unknown.png");
    ui_assets.heart_broken = asset_server.load("ui/heart_broken.png");
    ui_assets.heart_full = asset_server.load("ui/heart_full.png");
    ui_assets.element_fire = asset_server.load("ui/element_fire.png");
    ui_assets.element_grass = asset_server.load("ui/element_grass.png");
    ui_assets.element_water = asset_server.load("ui/element_water.png");
    ui_assets.tool_toilet = asset_server.load("ui/tool_toilet.png");
    ui_assets.tool_hand = asset_server.load("ui/tool_hand.png");
    ui_assets.tool_underwear = asset_server.load("ui/tool_underwear.png");
    ui_assets.fira_sans_bold = asset_server.load("fonts/FiraSans-Bold.ttf");
}
