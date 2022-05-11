use bevy::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HeldAssets(Default::default()));
        app.add_startup_system(hold_assets);
    }
}

struct HeldAssets(Vec<HandleUntyped>);

fn hold_assets(
    asset_server: Res<AssetServer>,
    mut held_assets: ResMut<HeldAssets>,
) {
    held_assets.0.extend([
        asset_server.load_untyped("sprites/block-tile.png"),
        asset_server.load_untyped("sprites/doofus.png"),
        asset_server.load_untyped("sprites/ina.png"),
        asset_server.load_untyped("sprites/door.png"),
        asset_server.load_untyped("sprites/gate.png"),
        asset_server.load_untyped("fonts/FiraSans-Bold.ttf"),
    ]);
}
