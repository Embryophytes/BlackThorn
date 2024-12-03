use bevy::app::Plugin;
use ui_resize::UIResizeHandler;

pub mod ui_resize;

pub struct ViewportEventHandlerPlugin;

impl Plugin for ViewportEventHandlerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(UIResizeHandler);
    }
}
