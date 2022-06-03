#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{default, App, DefaultPlugins, WindowDescriptor};
use snake::GamePlugin;

#[allow(unused_mut)]
fn main() {
    let mut app = App::new();
    let mut resolution = (800., 600.);

    #[cfg(target_arch = "wasm32")]
    {
        let document = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .document_element()
            .unwrap();
        resolution = (
            document.client_width() as f32,
            document.client_height() as f32,
        );
        app.add_plugin(snake::web::WebViewportAutoResizePlugin);
    }

    app.insert_resource(WindowDescriptor {
        title: String::from("Snake"),
        width: resolution.0,
        height: resolution.1,
        ..default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(GamePlugin)
    .run();
}
