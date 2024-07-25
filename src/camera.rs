use bevy::{asset::TrackAssets, prelude::*};

use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            // .add_systems(PostUpdate, movement_camera_with_player)
        ;
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// fn movement_camera_with_player(
//     mut camera: Query<&mut Transform, With<Camera>>,
//     // player: Query<&Transform, With<Player>>,
// ) {
//     // let Ok(mut transform_camera) = camera.get_single_mut() else {
//     //     return;
//     // };
//     // let Ok(tranform_player) = player.get_single() else {
//     //     return;
//     // };
//     // transform_camera.translation = tranform_player.translation;
// }
