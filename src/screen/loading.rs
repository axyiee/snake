use bevy::prelude::{Plugin, ResMut, State, SystemSet};

use crate::GameplayState;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_enter(GameplayState::Loading)
                .with_system(load_assets)
        );
    }
}

fn load_assets(mut state: ResMut<State<GameplayState>>) {
    // todo: load assets
    state.set(GameplayState::MainMenu).expect("Failed to redirect to main menu.");
}
