use bevy::prelude::*;

use crate::{player::Player, schedule::InGameSet};

const CAM_LERP_FACTOR: f32 = 2.;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_camera).add_systems(
            PostUpdate,
            movement_camera_with_player.after(InGameSet::EntityUpdates),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn movement_camera_with_player(
    mut camera_quety: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player_quety: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Ok(mut transform_camera) = camera_quety.get_single_mut() else {
        return;
    };
    let Ok(transform_player) = player_quety.get_single() else {
        return;
    };
    let direction = Vec3::new(
        transform_player.translation.x,
        transform_player.translation.y,
        transform_camera.translation.z,
    );
    transform_camera.translation = transform_camera
        .translation
        .lerp(direction, time.delta_seconds() * CAM_LERP_FACTOR);
}
