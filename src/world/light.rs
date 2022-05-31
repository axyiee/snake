use bevy::{prelude::{ParallelSystemDescriptorCoercion, Plugin, SystemSet, Commands, Transform}, pbr::{PointLightBundle, PointLight}, math::Vec3};

use crate::GameplayState;

crate::labels!(SpawnLight);

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_enter(GameplayState::Playing).with_system(spawn_light.label(SpawnLight)),
        );
    }
}

fn spawn_light(mut commands: Commands) {
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 25.0, 25.0)),
        point_light: PointLight {
            range: 200.0,
            intensity: 1500.0,
            ..Default::default()
        },
        ..Default::default()
    });
}
