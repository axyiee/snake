use crate::{import, labels, GameplayState};
use bevy::{
    pbr::StandardMaterial,
    prelude::{
        Assets, Commands, Mesh, ParallelSystemDescriptorCoercion, Plugin, ResMut, SystemSet,
    },
};

labels!(SpawnPlayer);

import!(entity, input);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(PlayerInputPlugin).add_system_set(
            SystemSet::on_enter(GameplayState::Playing)
                .with_system(spawn_player.label(SpawnPlayer)),
        );
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PlayerBundle::new(&mut meshes, &mut materials));
}
