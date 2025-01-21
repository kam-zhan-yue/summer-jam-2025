use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct UiAssets {
    pub heart_broken: Handle<Image>,
    pub heart_full: Handle<Image>,
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
    ui_assets.heart_broken = asset_server.load("ui/heart_broken.png");
    ui_assets.heart_full = asset_server.load("ui/heart_full.png");
}
