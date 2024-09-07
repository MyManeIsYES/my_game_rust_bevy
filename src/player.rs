use crate::{
    asset_loader::ImageAssets, collision::Collider, damage::Damage, health::Health, movement::*,
    progress_bar::ProgresBar, schedule::InGameSet, state::GameState,
};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

const PLAYER_MAX_SPEED: u32 = 100;
const PLAYER_BOOST: f32 = 20000.0;
const PLAYER_SIZE: Vec2 = Vec2::new(70.0, 70.0);
const PLAYER_RESISTANCE: f32 = 0.5;
const PLAYER_RADIUS: f32 = 17.0;
const PLAYER_DAMAGE: f32 = 35.0;
const PLAYER_HEALT: f32 = 1000.0;
const PLAYER_MASS: f32 = 200.0;
const PLAYER_ABSORPTION: f32 = 0.5;

#[derive(Component, Debug)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(OnEnter(GameState::GameOver), spawn_player)
            .add_systems(
                PreUpdate,
                movement_player_control.chain().in_set(InGameSet::UserInput),
            )
            .add_systems(Update, player_destroyed.in_set(InGameSet::EntityUpdates));
    }
}

fn spawn_player(
    mut commands: Commands,
    image_asset: Res<ImageAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(PLAYER_SIZE),
                    ..Default::default()
                },
                texture: image_asset.player_stay.clone(),

                transform: Transform::from_translation(Vec3::new(30.1, 0.0, 0.0)),

                ..default()
            },
            MovingObjectBundle {
                velocity: Velocity::new(Vec3::default(), PLAYER_MAX_SPEED),
                acceleration: Acceleration::new(Vec3::default()),
                collider: Collider::new(PLAYER_RADIUS, PLAYER_MASS,PLAYER_ABSORPTION),
            },
            Health::new(PLAYER_HEALT),
            Damage::new(PLAYER_DAMAGE),
            Player,
        ))
        // .insert(MaterialMesh2dBundle {
        //     mesh: Mesh2dHandle(meshes.add(Rectangle::new(50.0, 10.0))),
        //     material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
        //     ..default()
        // })
    ;
}

fn movement_player_control(
    mut query: Query<
        (
            &mut Transform,
            &mut Velocity,
            &mut Acceleration,
            &mut Handle<Image>,
        ),
        With<Player>,
    >,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    image_asset: Res<ImageAssets>,
) {
    let Ok((mut transform, mut velocity, mut acceleration, mut image)) = query.get_single_mut()
    else {
        return;
    };
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

    acceleration.value = boost.normalize_or_zero() * PLAYER_BOOST * time.delta_seconds();

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

    //change texture todo
    if velocity.value.x == 0.0 && velocity.value.y == 0.0 {
        *image = image_asset.player_stay.clone();
    } else {
        *image = image_asset.player_move.clone();
    }
}

fn player_destroyed(mut next_state: ResMut<NextState<GameState>>, query: Query<(), With<Player>>) {
    if query.get_single().is_err() {
        next_state.set(GameState::GameOver);
    }
}
