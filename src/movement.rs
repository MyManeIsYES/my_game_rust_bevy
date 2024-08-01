use bevy::prelude::*;

use crate::{collision::Collider, schedule::InGameSet};

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
    pub max: u32,
}

impl Velocity {
    pub fn new(value: Vec3, max: u32) -> Self {
        Self { value, max }
    }
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub collider: Collider,
}
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_velocity, update_position, update_max_velocity)
                .chain()
                .in_set(InGameSet::EntityUpdates),
        );
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}

fn update_max_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        if velocity.value.length() as u32 >= velocity.max {
            let projection_length: f32 = (velocity.value.x * acceleration.value.x
                + velocity.value.y * acceleration.value.y)
                / (velocity.value.x * velocity.value.x + velocity.value.y * velocity.value.y)
                    .sqrt();

            let resist_acceliration =
                -velocity.value.normalize_or_zero() * projection_length * time.delta_seconds();
            velocity.value += resist_acceliration;
        }
    }
}
