use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetCollectionApp};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<GraphicAssets>();
        app.init_collection::<FontAssets>();
    }
}

#[derive(AssetCollection)]
pub struct GraphicAssets {
    #[asset(path = "sprites/block-tile.png")]
    pub block_tile: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}
