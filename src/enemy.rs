use bevy::prelude::*;
use rand::Rng;

use crate::{
    asset_loader::ImageAssets, collision::Collider, damage::Damage, health::Health, movement::*,
    player::Player, schedule::InGameSet,
};

const SPAWN_TIME_SECONDS: f32 = 1.0;
const ENEMY_SIZE: Vec2 = Vec2::new(50.0, 50.0);
const MAX_COUNT_ENEMY: u32 = 10;
const ENEMY_MAX_SPEED: u32 = 500;
const SPAWN_RANGE: f32 = 500.0;
const ENEMY_BOOST: f32 = 1000.0;
const ENEMY_RADIUS: f32 = 20.0;
const ENEMY_GAMAGE: f32 = 30.0;
const ENEMY_HEALTH: f32 = 100.0;

#[derive(Component, Debug)]
pub struct Enemy;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

#[derive(Resource, Debug)]
pub struct CountEnemy {
    pub count: u32,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .insert_resource(CountEnemy { count: 0 })
        .add_systems(
            Update,
            (spawn_enemy, movement_enemy).in_set(InGameSet::EntityUpdates),
        )
        // .add_event::<OnAdd>()
        ;
    }
}

fn spawn_enemy(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    player: Query<&Transform, With<Player>>,
    time: Res<Time>,
    mut count_enemy: ResMut<CountEnemy>,
    image_assets: Res<ImageAssets>,
) {
    if count_enemy.count >= MAX_COUNT_ENEMY {
        return;
    }

    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let Ok(transform_player) = player.get_single() else {
        return;
    };

    let mut rng = rand::thread_rng();

    let number: f64 = rng.gen_range(0.0..6.283185);

    let translation = Vec3::new(
        (number.cos() as f32) * SPAWN_RANGE + transform_player.translation.x,
        (number.sin() as f32) * SPAWN_RANGE + transform_player.translation.y,
        0.0,
    );

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(ENEMY_SIZE),
                ..default()
            },
            transform: Transform::from_translation(translation),
            texture: image_assets.enemy.clone(),
            ..default()
        },
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::default(), ENEMY_MAX_SPEED),
            acceleration: Acceleration::new(Vec3::default()),
            collider: Collider::new(ENEMY_RADIUS),
        },
        Health::new(ENEMY_HEALTH),
        Damage::new(ENEMY_GAMAGE),
        Enemy,
    ));
    count_enemy.count += 1;
}

fn movement_enemy(
    mut query_enemy: Query<(&Transform, &mut Acceleration), With<Enemy>>,
    player: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(transform_player) = player.get_single() else {
        return;
    };
    for (transform, mut acceleration) in query_enemy.iter_mut() {
        let translation_player = transform_player.translation - transform.translation;
        let movement = Vec3::new(translation_player.x, translation_player.y, 0.0);
        acceleration.value = movement.normalize_or_zero() * ENEMY_BOOST * time.delta_seconds();
    }
}
