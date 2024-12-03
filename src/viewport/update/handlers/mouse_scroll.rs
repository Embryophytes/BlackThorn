use bevy::app::Plugin;
use bevy::app::Update;

pub struct MouseScrollHandler;

impl Plugin for MouseScrollHandler {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, mouse_wheel_scroll);
    }
}

fn mouse_wheel_scroll() {}
