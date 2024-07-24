use bevy::prelude::*;

use crate::movement::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_systems(Starup, spawn_lines)
            .add_systems(PostUpdate, print_position);
    }
}

fn print_position(query: Query<(Entity, &Transform, &Velocity, &Acceleration)>) {
    // Log the entity ID and translation of each entity with a `Position` component.
    for (entity, transform, velocity, acceleration) in query.iter() {
        info!(
            "Entity {:?} is at transform {:?}, velocity {:?}, acceleration {:?}",
            entity, transform, velocity.value, acceleration.value
        );
    }
}
