use bevy::{
    prelude::{App, Plugin, ResMut},
    window::Windows,
};

struct WebViewportState(futures::channel::mpsc::UnboundedReceiver<(f32, f32)>);

pub struct WebViewportAutoResizePlugin;

impl Plugin for WebViewportAutoResizePlugin {
    fn build(&self, app: &mut App) {
        let (sender, receiver) = futures::channel::mpsc::unbounded();
        let window = web_sys::window().expect("no global `window` exists");

        gloo_events::EventListener::new(&window, "resize", move |_| {
            let element = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .document_element()
                .unwrap();
            let width = element.client_width() as f32;
            let height = element.client_height() as f32;
            sender.unbounded_send((width, height)).unwrap();
        })
        .forget();

        app.insert_resource(WebViewportState(receiver))
            .add_system(auto_resize);
    }
}

fn auto_resize(mut windows: ResMut<Windows>, mut state: ResMut<WebViewportState>) {
    if let Ok(Some(resolution)) = state.0.try_next() {
        if let Some(window) = windows.get_primary_mut() {
            window.set_resolution(resolution.0, resolution.1);
        }
    }
}
