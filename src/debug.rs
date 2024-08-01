use bevy::prelude::*;

use crate::{health::Health, movement::*, schedule::InGameSet};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_systems(Starup, spawn_lines)
            .add_systems(PostUpdate, print_position.after(InGameSet::EntityUpdates));
    }
}

fn print_position(query: Query<(Entity, &Transform, &Velocity, &Acceleration, &Health)>) {
    // Log the entity ID and translation of each entity with a `Position` component.
    for (entity, transform, velocity, acceleration, health) in query.iter() {
        info!(
            "Entity {:?} is at transform {:?}, velocity {:?}, acceleration {:?}, health {:?}",
            entity, transform, velocity.value, acceleration.value, health.value,
        );
    }
}
