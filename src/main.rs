mod asset_loader;
mod background;
mod camera;
mod collision;
mod damage;
mod debug;
mod despawn;
mod enemy;
mod health;
mod movement;
mod player;
mod schedule;
mod state;

use asset_loader::AssetLoaderPlugin;
use background::BackgroundPLugin;
use bevy::prelude::*;
use bevy_dev_tools::fps_overlay::FpsOverlayPlugin;
use camera::CameraPlugin;
use collision::CollisionPlugin;
use debug::DebugPlugin;
use despawn::DespawnPlugin;
use enemy::EnemyPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use schedule::SchedulePlugin;
use state::StatePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FpsOverlayPlugin::default())
        //User Plugins
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(BackgroundPLugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(AssetLoaderPlugin)
        .run();
}
