use crate::import;
use bevy::prelude::Plugin;

import!(main_menu, loading);

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(LoadingScreenPlugin)
            .add_plugin(MainMenuScreenPlugin);
    }
}
