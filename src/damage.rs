use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Damage {
    pub amount: f32,
}

impl Damage {
    pub fn new(amount: f32) -> Self {
        Self { amount }
    }
}
