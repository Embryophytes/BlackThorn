use bevy::app::Plugin;
use dragg_tables::DraggTablesHandler;
use mouse_button::MouseButtonHandler;
use mouse_scroll::MouseScrollHandler;
use select_tables::SelectTablesHandler;
use ui_resize::UIResizeHandler;

pub mod dragg_tables;
pub mod mouse_button;
pub mod mouse_scroll;
pub mod select_tables;
pub mod ui_resize;

pub struct ViewportEventHandlerPlugin;

impl Plugin for ViewportEventHandlerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(UIResizeHandler)
            .add_plugins(MouseButtonHandler)
            .add_plugins(MouseScrollHandler)
            .add_plugins(DraggTablesHandler)
            .add_plugins(SelectTablesHandler);
    }
}
