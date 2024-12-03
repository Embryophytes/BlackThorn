use bevy::app::Plugin;
use mouse_movement::MouseMovementHandler;
use mouse_scroll::MouseScrollHandler;
use ui_resize::UIResizeHandler;

pub mod mouse_movement;
pub mod mouse_scroll;
pub mod ui_resize;

pub struct ViewportEventHandlerPlugin;

impl Plugin for ViewportEventHandlerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(UIResizeHandler)
            .add_plugins(MouseMovementHandler)
            .add_plugins(MouseScrollHandler);
    }
}
