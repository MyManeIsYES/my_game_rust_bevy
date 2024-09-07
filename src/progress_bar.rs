use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component, Debug)]
pub struct ProgresBar {
    pub shape: Mesh2dHandle,
    pub color: Color,
    pub transform: Transform,
}

pub struct ProgressBarPlugin;

impl Plugin for ProgressBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup)
            // .add_systems(Update, update_progress_bar)
        ;
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(50.0, 10.0))),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
        ..default()
    });
}
