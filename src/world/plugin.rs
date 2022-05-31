use crate::{import, palette::ColorPalette};
use bevy::{core_pipeline::ClearColor, prelude::{Plugin, Msaa}};

import!(camera);
import!(light);

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ClearColor(ColorPalette::Background.color()))
            .insert_resource(Msaa::default())
            .add_plugin(CameraPlugin)
            .add_plugin(LightPlugin);
    }
}
