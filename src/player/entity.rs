use super::{SnakeInputView, SnakeTypeBindings, InputBundle};
use bevy::{
    ecs::bundle,
    pbr::{PbrBundle, StandardMaterial},
    prelude::{default, shape, Assets, Bundle, Color, Component, Mesh, ResMut, Transform},
};

#[derive(Debug, Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    #[bundle]
    pub input: InputBundle,
    #[bundle]
    pub pbr: PbrBundle,
}

impl PlayerBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        Self {
            marker: Player,
            input: InputBundle::default(),
            pbr: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 10.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default()
            },
        }
    }
}
