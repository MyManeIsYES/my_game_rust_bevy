use bevy::prelude::*;
use rand::Rng;

use crate::{
    movement::{Acceleration, Velocity},
    player::Player,
};

const SPAWN_TIME_SECONDS: f32 = 1.0;
const ENEMY_SIZE: Vec2 = Vec2::new(50.0, 50.0);
const MAX_COUNT_ENEMY: u32 = 10;
const ENEMY_SPEED: f32 = 500.0;
const SPAWN_RANGE: f32 = 500.0;

#[derive(Component, Debug)]
pub struct Enemy;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(Update, spawn_enemy)
        .add_systems(PostUpdate, movement_enemy);
    }
}

fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    let number: f64 = rng.gen_range(0.0..6.283185);

    let translation = Vec3::new(
        (number.cos() as f32) * SPAWN_RANGE,
        (number.sin() as f32) * SPAWN_RANGE,
        0.0,
    );

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(ENEMY_SIZE),
                ..default()
            },
            transform: Transform::from_translation(translation),
            texture: asset_server.load("skeleton-skull.png"),
            ..default()
        },
        Velocity {
            value: Vec3::default(),
        },
        Acceleration {
            value: Vec3::default(),
        },
        Enemy,
    ));
}

fn movement_enemy(
    mut query_enemy: Query<(&Transform, &mut Velocity), With<Enemy>>,
    player: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(transform_player) = player.get_single() else {
        return;
    };
    for (transform, mut velocity) in query_enemy.iter_mut() {
        let translation_player = transform_player.translation - transform.translation;
        if translation_player.length() != 0.0 {
            let c: f32 = ENEMY_SPEED
                / (translation_player.x * translation_player.x
                    + translation_player.y * translation_player.y)
                    .sqrt();
            let movement = Vec3::new(translation_player.x, translation_player.y, 0.0);
            velocity.value = movement * c * time.delta_seconds();
        }
    }
}
