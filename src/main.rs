#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{default, App, DefaultPlugins, Msaa, WindowDescriptor};
use snake::GamePlugin;

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .insert_resource(WindowDescriptor {
            title: String::from("Snake"),
            height: 800.,
            width: 600.,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
