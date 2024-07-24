mod camera;
mod debug;
mod enemy;
mod movement;
mod player;

use bevy::prelude::*;
use bevy_dev_tools::fps_overlay::FpsOverlayPlugin;
use camera::CameraPlugin;
use debug::DebugPlugin;
use enemy::EnemyPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;

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
        .run();
}
