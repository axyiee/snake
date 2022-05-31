use bevy::prelude::{Plugin, ResMut, State, SystemSet};

use crate::GameplayState;

pub struct MainMenuScreenPlugin;

impl Plugin for MainMenuScreenPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_enter(GameplayState::MainMenu)
                .with_system(|mut state: ResMut<State<GameplayState>>| {
                    // to-do: main menu logic
                    state.set(GameplayState::Playing).expect("Failed to redirect to game.");
                })
        );
    }
}