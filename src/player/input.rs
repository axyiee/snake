use crate::GameplayState;

#[allow(clippy::module_inception)]
mod input {
    pub use bevy::prelude::{
        GamepadAxisType, GamepadButtonType, KeyCode, ParallelSystemDescriptorCoercion, Plugin
    };
    pub use ezinput::prelude::*;
}

use bevy::prelude::{SystemSet, Query, ParallelSystemDescriptorCoercion, Res, With, Transform, Time};
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

fn handle_input(mut query: Query<(&SnakeInputView, &mut Transform), With<Player>>, time: Res<Time>) {
    for (view, mut transform) in query.iter_mut() {
        let view: &SnakeInputView = view;

        if let Some(axis) = view.axis(&SnakeTypeBindings::Movement(TypeMovement::Horizontal)).first() {
            if axis.press != PressState::Released {
                transform.translation.x += axis.value * 15. * time.delta_seconds();
            }
        }
        if let Some(axis) = view.axis(&SnakeTypeBindings::Movement(TypeMovement::Vertical)).first() {
            if axis.press != PressState::Released {
                transform.translation.y += axis.value * 15. * time.delta_seconds();
            }
        }
    }
}

