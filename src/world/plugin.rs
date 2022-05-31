use crate::{import, palette::ColorPalette};
use bevy::{core_pipeline::ClearColor, prelude::Plugin};

import!(camera);

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ClearColor(ColorPalette::Background.color()))
            .add_plugin(CameraPlugin);
    }
}
