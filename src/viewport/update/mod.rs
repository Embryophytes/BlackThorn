use bevy::app::Plugin;
use handlers::ViewportEventHandlerPlugin;

pub mod handlers;

pub struct UpdateViewportPlugin;

impl Plugin for UpdateViewportPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(ViewportEventHandlerPlugin);
    }
}
