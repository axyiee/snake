use crate::{macros::ez, GameplayState};
mod input {
    pub use bevy::prelude::{GamepadAxisType, GamepadButtonType, KeyCode, Plugin};
    pub use ezinput::prelude::{BindingInputReceiver::*, *};
    pub use ezinput_macros::*;
}

use super::entity::Player;
use bevy::prelude::{Commands, Entity, Query, SystemSet, With};
use input::*;

ez! {
    SnakeTypeBindings {
        Movement<TypeMovement> {
            Horizontal = [
                KeyboardKey(KeyCode::Left) => -1., KeyboardKey(KeyCode::A) => -1.,
                KeyboardKey(KeyCode::Right), KeyboardKey(KeyCode::D),
                GamepadAxis(GamepadAxisType::LeftStickX),
            ],
            Vertical = [
                KeyboardKey(KeyCode::Up), KeyboardKey(KeyCode::W),
                KeyboardKey(KeyCode::Down) => -1., KeyboardKey(KeyCode::S) => -1.,
                GamepadAxis(GamepadAxisType::LeftStickY),
            ],
        },
    }
}

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(EZInputPlugin::<SnakeTypeBindings>::default())
            .add_system_set(
                SystemSet::on_enter(GameplayState::Playing).with_system(insert_player_input),
            );
    }
}

fn insert_player_input(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(SnakeTypeBindings::view());
    }
}
