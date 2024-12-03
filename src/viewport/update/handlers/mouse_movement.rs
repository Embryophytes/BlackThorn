use bevy::app::Plugin;
use bevy::app::Update;

pub struct MouseMovementHandler;

impl Plugin for MouseMovementHandler {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, mouse_right_button_hold);
    }
}

fn mouse_right_button_hold() {}
