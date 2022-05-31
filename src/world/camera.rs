use bevy::prelude::{Plugin, SystemSet, ParallelSystemDescriptorCoercion, Commands, PerspectiveCameraBundle, UiCameraBundle};

use crate::GameplayState;

crate::labels!(SpawnPlayerCamera, SpawnUiCamera);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_ui_camera.label(SpawnUiCamera));
        app.add_system_set(SystemSet::on_enter(GameplayState::Playing).with_system(spawn_player_camera.label(SpawnPlayerCamera)));
    }
}

fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn spawn_player_camera(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle::default());
}

