use bevy::{asset::io::memory::Value, prelude::*, utils::HashMap};

use crate::{
    damage::Damage, enemy::Enemy, health::Health, movement::Velocity, player::Player,
    schedule::InGameSet,
};

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub mass: f32,
    pub absorption_coefficient: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32, mass: f32, absorption_coefficient: f32) -> Self {
        Self {
            radius,
            mass,
            absorption_coefficient,
            colliding_entities: vec![],
        }
    }
}

#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub entity: Entity,
    pub collided_entity: Entity,
}

impl CollisionEvent {
    pub fn new(entity: Entity, collided_entity: Entity) -> Self {
        Self {
            entity,
            collided_entity,
        }
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            collision_detection.in_set(InGameSet::CollisionDetection),
        )
        .add_systems(
            Update,
            (
                (handle_collisions::<Enemy>, handle_collisions::<Player>),
                // apply_collision_damage,
                apply_collision_fix_position,
                // apply_collision_fix_velocity,
            )
                .chain()
                .in_set(InGameSet::EntityUpdates),
        )
        .add_event::<CollisionEvent>();
    }
}

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    // First phase: Detect collisions.
    for (entity_a, transform_a, collider_a) in query.iter() {
        for (entity_b, transform_b, collider_b) in query.iter() {
            if entity_a != entity_b {
                let distance = transform_a
                    .translation()
                    .distance(transform_b.translation());
                if distance <= collider_a.radius + collider_b.radius {
                    colliding_entities
                        .entry(entity_a)
                        .or_insert_with(Vec::new)
                        .push(entity_b);
                }
            }
        }
    }

    // Second phase: Update colliders.
    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider
                .colliding_entities
                .extend(collisions.iter().copied());
        }
    }
}

fn handle_collisions<T: Component>(
    mut collision_event_writer: EventWriter<CollisionEvent>,
    query: Query<(Entity, &Collider), With<T>>,
) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            // Entity collided with another entity of the same type.
            if query.get(collided_entity).is_ok() {
                continue;
            }
            // Send collision event.
            collision_event_writer.send(CollisionEvent::new(entity, collided_entity));
        }
    }
}

fn apply_collision_damage(
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut health_query: Query<&mut Health>,
    collision_damage_query: Query<&Damage>,
) {
    for &CollisionEvent {
        entity,
        collided_entity,
    } in collision_event_reader.read()
    {
        let Ok(mut health) = health_query.get_mut(entity) else {
            continue;
        };

        let Ok(collision_damage) = collision_damage_query.get(collided_entity) else {
            continue;
        };

        // Apply any damage that should be dealt as a result of the collision.
        health.value -= collision_damage.amount;
    }
}

fn apply_collision_fix_position(
    // mut collision_event_reader: EventReader<CollisionEvent>,
    mut query: Query<(Entity, &mut Transform, &mut Velocity, &Collider)>,
    time: Res<Time>,
) {
    let mut iter = query.iter_combinations_mut();
    while let Some(
        [(entity_a, mut transform_a, mut velocity_a, collider_a), (entity_b, mut transform_b, mut velocity_b, collider_b)],
    ) = iter.fetch_next()
    {
        if entity_a == entity_b {
            continue;
        }

        let distance = (transform_a.translation - transform_b.translation).length();

        let collision_distance = collider_b.radius + collider_a.radius - distance;

        if collision_distance < 0.0 {
            continue;
        }
        //fix translation
        let a = (transform_a.translation - transform_b.translation).normalize_or_zero()
            * (collision_distance / 2.0)
            * (collider_b.mass / collider_a.mass);
        let b = (transform_b.translation - transform_a.translation).normalize_or_zero()
            * (collision_distance / 2.0)
            * (collider_a.mass / collider_b.mass);

        transform_a.translation += a;
        transform_b.translation += b;

        //fix velocity
        //3

        // let length_a = velocity_a.value.length();
        // let length_b = velocity_b.value.length();

        // let const_p = collider_a.mass * length_a + collider_b.mass * length_b;

        // let new_length_a = ((const_p - collider_b.mass * length_b) / collider_a.mass)
        //     * collider_a.absorption_coefficient;

        // let new_length_b = ((const_p - collider_a.mass * length_a) / collider_b.mass)
        //     * collider_b.absorption_coefficient;

        // let b_a = transform_a.translation + velocity_a.value;
        // velocity_a.value = ((transform_b.translation * 2.0 - a)
        //     - (transform_b.translation * 2.0 - velocity_a.value))
        //     .normalize_or_zero()
        //     * new_length_a
        //     * time.delta_seconds();

        // let b = transform_b.translation + velocity_b.value;
        // velocity_b.value = ((transform_a.translation * 2.0 - b)
        //     - (transform_a.translation * 2.0 - velocity_b.value))
        //     .normalize_or_zero()
        //     * new_length_b
        //     * time.delta_seconds();

        //2
        //         let length_a = velocity_a.value.length();
        // let length_b = velocity_b.value.length();

        // let const_p = collider_a.mass * length_a + collider_b.mass * length_b;

        // let new_length_a = (const_p - collider_b.mass * length_b) / collider_a.mass
        //     * collider_a.absorption_coefficient;
        // let new_length_b = (const_p - collider_a.mass * length_a) / collider_b.mass
        //     * collider_b.absorption_coefficient;
        //2.2
        // let angel_a_alpha: f32 = (velocity_a.value - velocity_b.value)
        //     .angle_between(Vec3::new(1., 0., 0.))
        //     - 1.470796327;

        //2.1
        // let angel_a = velocity_a.value.angle_between(Vec3::new(1., 0., 0.));
        // let angel_b = velocity_b.value.angle_between(Vec3::new(1., 0., 0.));

        // let angle_spec_a: f32 = angel_b - 1.470796327;
        // let angle_spec_b: f32 = angel_a - 1.470796327;cc

        //1
        let vec_p = transform_a.translation - transform_b.translation;
        //
        let vec_a_x: Vec3 = vec_p.normalize_or_zero()
            * ((velocity_a.value.x * vec_p.x + velocity_a.value.y * vec_p.y) / vec_p.length());
        let vec_b_x: Vec3 = vec_p.normalize_or_zero()
            * ((velocity_b.value.x * vec_p.x + velocity_b.value.y * vec_p.y) / vec_p.length());
        //
        let const_p = collider_a.mass * vec_a_x.length() + collider_b.mass * vec_b_x.length();

        let new_length_a = (const_p - collider_b.mass * vec_b_x.length()) / collider_a.mass
            * collider_a.absorption_coefficient;
        let new_length_b = (const_p - collider_a.mass * vec_a_x.length()) / collider_b.mass
            * collider_b.absorption_coefficient;
        //
        velocity_a.value += -(vec_a_x.normalize_or_zero() * new_length_a * 2.0);
        velocity_b.value += -(vec_b_x.normalize_or_zero() * new_length_b * 2.0);
    }
}
