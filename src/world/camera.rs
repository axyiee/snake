use bevy::{
    math::Vec3,
    prelude::{
        default, Commands, ParallelSystemDescriptorCoercion, Plugin,
        SystemSet, Transform, Camera3dBundle, OrthographicProjection,
    },
};

use crate::GameplayState;

crate::labels!(SpawnPlayerCamera);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_enter(GameplayState::Playing)
                .with_system(spawn_player_camera.label(SpawnPlayerCamera)),
        );
    }
}

fn spawn_player_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(50.0, 15.0, 50.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        projection: OrthographicProjection::default().into(),
        ..default()
    });
}
