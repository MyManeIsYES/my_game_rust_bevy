use bevy::prelude::*;

pub struct BackgroundPLugin;

impl Plugin for BackgroundPLugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, spawn_background);
    }
}

fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut materials: ResMut<Assets<>>,
) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("The_Creation_of_Adam.png"),
        ..default()
    });
}
