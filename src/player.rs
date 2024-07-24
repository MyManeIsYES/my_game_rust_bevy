use std::f32::consts::SQRT_2;

use crate::movement::*;
use bevy::prelude::*;

const PLAYER_MAX_SPEED: f32 = 1000.0;
const PLAYER_BOOST: f32 = 10000.0;
const PLAYER_SIZE: Vec2 = Vec2::new(70.0, 70.0);
const PLAYER_RESISTANCE: f32 = 0.5;

#[derive(Component, Debug)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(PreUpdate, player_movement_control);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            texture: asset_server.load("YOU.png"),
            ..default()
        },
        Velocity {
            value: Vec3::default(),
        },
        Acceleration {
            value: Vec3::default(),
        },
        Player,
    ));
}

fn player_movement_control(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Velocity, &mut Acceleration), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut transform, mut velocity, mut acceleration) in &mut query.iter_mut() {
        //contral player
        let mut boost: Vec3 = Vec3::new(0.0, 0.0, 0.0);

        if keyboard_input.pressed(KeyCode::KeyW) {
            boost.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            boost.y += -1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            boost.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            boost.x += -1.0;
        }
        if boost.length() != 0.0 {
            let c: f32 = PLAYER_BOOST / (boost.x * boost.x + boost.y * boost.y).sqrt();
            boost.x *= (c * time.delta_seconds());
            boost.y *= (c * time.delta_seconds());
        }
        acceleration.value = boost;

        //player resistance
        if velocity.value.length() > 0.0 && acceleration.value.length() == 0.0 {
            let resist = Vec3::new(velocity.value.x, velocity.value.y, velocity.value.z);
            velocity.value -= (resist * PLAYER_RESISTANCE) * time.delta_seconds();

            if velocity.value.length() < 5.0 {
                velocity.value *= 0.0;
            }
        }

        // player angle
        let diff = velocity.value;
        let angle = diff.y.atan2(diff.x);
        transform.rotation = Quat::from_rotation_z(angle);

        //extra stop
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            acceleration.value *= 0.0;
            velocity.value *= 0.0;
            transform.rotation = Quat::from_rotation_z(0.0);
        }

        //change texture
        if velocity.value.x == 0.0 && velocity.value.y == 0.0 {
            commands.entity(entity).insert(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(PLAYER_SIZE),
                    ..default()
                },
                transform: *transform,
                texture: asset_server.load("YOU.png"),
                ..default()
            });
        } else {
            commands.entity(entity).insert(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(PLAYER_SIZE),
                    ..default()
                },
                transform: *transform,
                texture: asset_server.load("NOU.png"),
                ..default()
            });
        }
    }
}
