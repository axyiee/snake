use crate::GameplayState;

#[allow(clippy::module_inception)]
mod input {
    pub use bevy::prelude::{
        GamepadAxisType, GamepadButtonType, KeyCode, ParallelSystemDescriptorCoercion, Plugin
    };
    pub use ezinput::prelude::{*, InputReceiver::*};
}

use bevy::prelude::{SystemSet, Query, ParallelSystemDescriptorCoercion, With, Transform};
use input::*;

use super::Player;

input! {
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

pub type SnakeInputView = InputView<SnakeTypeBindings>;

crate::labels!(HandlePlayerInput);

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(EZInputPlugin::<SnakeTypeBindings>::default())
            .add_system_set(
                SystemSet::on_update(GameplayState::Playing)
                    .with_system(handle_input.label(HandlePlayerInput)),
            );
    }
}

fn handle_input(mut query: Query<(&SnakeInputView, &mut Transform), With<Player>>) {
    for (view, mut transform) in query.iter_mut() {
        let view: &SnakeInputView = view;

        if let Some(left_axis) = view.axis(&SnakeTypeBindings::Movement(TypeMovement::Horizontal)).first() {
            if left_axis.1 != PressState::Released {
                transform.translation.x += left_axis.0 * 15. * (1. / 60.);
            }
        }
        if let Some(left_axis) = view.axis(&SnakeTypeBindings::Movement(TypeMovement::Vertical)).first() {
            if left_axis.1 != PressState::Released {
                transform.translation.y += left_axis.0 * 15. * (1. / 60.);
            }
        }
    }
}

