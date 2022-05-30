use crate::import;
use bevy::prelude::Plugin;

import!(entity, input);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(PlayerInputPlugin);
    }
}
