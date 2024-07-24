use bevy::{color::palettes::css::DEEP_PINK, prelude::*};
use rand::Rng;

use crate::{
    movement::{Acceleration, Velocity},
    player::Player,
};

const SPAWN_TIME_SECONDS: f32 = 1.0;
const ENEMY_SIZE: Vec2 = Vec2::new(50.0, 50.0);

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
        // .add_systems(Update, spawn_enemy)
        ;
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

    let translation = Vec3::new(
        rng.gen_range(-600.0..600.0),
        rng.gen_range(-300.0..300.0),
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

fn enemy_movement(
    mut query_enemy: Query<&mut Velocity, With<Enemy>>,
    mut query_player: Query<&Transform, With<Player>>,
) {
}
