use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Damage {
    pub value: f32,
}

impl Damage {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}
