use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetCollectionApp};
use bevy_yoleck::YoleckLevelIndex;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<GameAssets>();
    }
}

#[derive(AssetCollection)]
pub struct GameAssets {
    #[asset(path = "sprites/block-tile.png")]
    pub block_tile: Handle<Image>,
    #[asset(path = "sprites/doofus.png")]
    pub doofus: Handle<Image>,
    #[asset(path = "sprites/ina.png")]
    pub ina: Handle<Image>,
    #[asset(path = "sprites/door.png")]
    pub door: Handle<Image>,
    #[asset(path = "sprites/gate.png")]
    pub gate: Handle<Image>,
    #[asset(path = "sprites/crystal-off.png")]
    pub crystal_off: Handle<Image>,
    #[asset(path = "sprites/crystal-on.png")]
    pub crystal_on: Handle<Image>,
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub font: Handle<Font>,
    #[asset(path = "levels/index.yoli")]
    pub level_index: Handle<YoleckLevelIndex>,
}
