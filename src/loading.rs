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
    #[asset(path = "sprites/floor-block.png")]
    pub floor_block: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}
