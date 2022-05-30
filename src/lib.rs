use bevy::prelude::{App, Plugin};

pub mod math;

#[path = "player/plugin.rs"]
pub mod player;

#[path = "macro/mod.rs"]
mod macros;

#[cfg(target_arch = "wasm32")]
pub mod web;

/// As recommended by usual game developers, we'll use states to separate
/// logic in this game. Bevy has a built-in state system, so we'll use that:
/// https://bevy-cheatbook.github.io/programming/states.html
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameplayState {
    Loading,
    Playing,
    MainMenu,
}

impl GameplayState {
    /// This function returns a description for each state.
    pub fn description(&self) -> &'static str {
        match &self {
            Self::Loading => "Loading SFX assets...",
            _ => "This state doesn't have a description.",
        }
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameplayState::Loading)
            .add_plugin(player::PlayerPlugin);

        #[cfg(debug_assertions)]
        {
            use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
