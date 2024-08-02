use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct ImageAssets {
    pub player_move: Handle<Image>,
    pub player_stay: Handle<Image>,
    pub enemy: Handle<Image>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ImageAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<ImageAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = ImageAssets {
        player_move: asset_server.load("NOU.png"),
        player_stay: asset_server.load("YOU.png"),
        enemy: asset_server.load("skeleton-skull.png"),
    }
}
