use bevy::{prelude::*, utils::HashMap};

use crate::{
    combo::ResolveResult,
    schedule::GameSet,
    types::{Action, Choice, Element, Outcome},
};

#[derive(Component, Debug)]
pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameAssets>();
        app.add_systems(PreStartup, setup_game_assets.in_set(GameSet::Flow));
    }
}

pub struct GlobalPlugin;

impl Plugin for GlobalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiAssets>();
        app.init_resource::<GameAssets>();
        app.init_resource::<AudioAssets>();
        app.add_systems(
            PreStartup,
            (setup_ui_assets, setup_game_assets, setup_audio_assets).in_set(GameSet::Flow),
        );
    }
}

#[derive(Resource, Debug, Default)]
pub struct GameAssets {
    pub player_one: Handle<Image>,
    pub player_two: Handle<Image>,
    pub aura_fire: Handle<Image>,
    pub aura_water: Handle<Image>,
    pub aura_grass: Handle<Image>,
}

#[derive(Resource, Debug, Default)]
pub struct AudioAssets {
    pub nuggie: Vec<Handle<AudioSource>>,
    pub wedgie: Vec<Handle<AudioSource>>,
    pub swirly: Vec<Handle<AudioSource>>,
    pub laugh: Vec<Handle<AudioSource>>,
    pub ready: Handle<AudioSource>,
    pub player_one_wins: Handle<AudioSource>,
    pub player_two_wins: Handle<AudioSource>,
    pub draw: Vec<Handle<AudioSource>>,
    pub select_fire: Handle<AudioSource>,
    pub select_grass: Handle<AudioSource>,
    pub select_water: Handle<AudioSource>,
    pub select_generic: Vec<Handle<AudioSource>>,
    pub player_one_advantage: Handle<AudioSource>,
    pub player_two_advantage: Handle<AudioSource>,
    pub no_advantage: Handle<AudioSource>,
    pub fight: Handle<AudioSource>,
    pub select_element: Handle<AudioSource>,
    pub combo_breaker: Handle<AudioSource>,
}

#[derive(Resource, Debug, Default)]
pub struct UiAssets {
    pub logo: Handle<Image>,
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
    pub result_swirly_p1: Handle<Image>,
    pub result_swirly_p2: Handle<Image>,
    pub result_whirly_p1: Handle<Image>,
    pub result_whirly_p2: Handle<Image>,
    pub result_wedgie_p1: Handle<Image>,
    pub result_wedgie_p2: Handle<Image>,
    pub result_draw: Handle<Image>,
    pub fira_sans_bold: Handle<Font>,
    pub ms_pain: Handle<Font>,
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

    pub fn get_result(&self, result: ResolveResult) -> Handle<Image> {
        match (result.outcome, result.choice) {
            (Outcome::PlayerOne, Choice::Action(Action::Hand)) => self.result_whirly_p1.clone(),
            (Outcome::PlayerOne, Choice::Action(Action::Toilet)) => self.result_swirly_p1.clone(),
            (Outcome::PlayerOne, Choice::Action(Action::Underwear)) => {
                self.result_wedgie_p1.clone()
            }
            (Outcome::PlayerTwo, Choice::Action(Action::Hand)) => self.result_whirly_p2.clone(),
            (Outcome::PlayerTwo, Choice::Action(Action::Toilet)) => self.result_swirly_p2.clone(),
            (Outcome::PlayerTwo, Choice::Action(Action::Underwear)) => {
                self.result_wedgie_p2.clone()
            }
            _ => self.result_draw.clone(),
        }
    }
}

fn setup_ui_assets(asset_server: Res<AssetServer>, mut ui_assets: ResMut<UiAssets>) {
    ui_assets.logo = asset_server.load("ui/game_logo.png");
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
    ui_assets.ms_pain = asset_server.load("fonts/MS_PAIN.ttf");
    ui_assets.result_swirly_p1 = asset_server.load("ui/result_swirly_p1.png");
    ui_assets.result_swirly_p2 = asset_server.load("ui/result_swirly_p2.png");
    ui_assets.result_wedgie_p1 = asset_server.load("ui/result_wedgie_p1.png");
    ui_assets.result_wedgie_p2 = asset_server.load("ui/result_wedgie_p2.png");
    ui_assets.result_whirly_p1 = asset_server.load("ui/result_whirly_p1.png");
    ui_assets.result_whirly_p2 = asset_server.load("ui/result_whirly_p1.png");
    ui_assets.result_draw = asset_server.load("ui/result_draw.png");
}

fn setup_game_assets(asset_server: Res<AssetServer>, mut game_assets: ResMut<GameAssets>) {
    game_assets.player_one = asset_server.load("sprites/stick_left.png");
    game_assets.player_two = asset_server.load("sprites/stick_right.png");
}

fn setup_audio_assets(asset_server: Res<AssetServer>, mut audio_assets: ResMut<AudioAssets>) {
    audio_assets
        .laugh
        .push(asset_server.load("audio/laugh_1.ogg"));
    audio_assets
        .laugh
        .push(asset_server.load("audio/laugh_2.ogg"));
    audio_assets
        .laugh
        .push(asset_server.load("audio/laugh_3.ogg"));

    audio_assets
        .nuggie
        .push(asset_server.load("audio/nuggie_1.ogg"));

    audio_assets
        .nuggie
        .push(asset_server.load("audio/nuggie_2.ogg"));

    audio_assets
        .nuggie
        .push(asset_server.load("audio/nuggie_3.ogg"));

    audio_assets
        .swirly
        .push(asset_server.load("audio/swirly_1.ogg"));

    audio_assets
        .swirly
        .push(asset_server.load("audio/swirly_2.ogg"));

    audio_assets
        .wedgie
        .push(asset_server.load("audio/wedgie_1.ogg"));

    audio_assets.player_one_wins = asset_server.load("audio/player_1_wins.ogg");
    audio_assets.player_two_wins = asset_server.load("audio/player_2_wins.ogg");

    audio_assets.ready = asset_server.load("audio/ready.ogg");
    audio_assets
        .draw
        .push(asset_server.load("audio/draw_1.ogg"));
    audio_assets
        .draw
        .push(asset_server.load("audio/draw_2.ogg"));

    audio_assets.select_fire = asset_server.load("audio/select_fire.ogg");
    audio_assets.select_grass = asset_server.load("audio/select_grass.ogg");
    audio_assets.select_water = asset_server.load("audio/select_water.ogg");
    audio_assets
        .select_generic
        .push(asset_server.load("audio/select_generic_1.ogg"));
    audio_assets
        .select_generic
        .push(asset_server.load("audio/select_generic_2.ogg"));
    audio_assets.select_element = asset_server.load("audio/announce_select_element.ogg");
    audio_assets.fight = asset_server.load("audio/announce_fight.ogg");
    audio_assets.combo_breaker = asset_server.load("audio/announce_combo.ogg");
    audio_assets.player_one_advantage = asset_server.load("audio/announce_one_adv.ogg");
    audio_assets.player_two_advantage = asset_server.load("audio/announce_two_adv.ogg");
    audio_assets.no_advantage = asset_server.load("audio/announce_no_adv.ogg");
}
