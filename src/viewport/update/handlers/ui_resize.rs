use bevy::prelude::*;

use bevy::app::Update;
use bevy::app::Plugin;
use bevy::window::PrimaryWindow;

use crate::ui::core::state::UIState;
use crate::viewport::core::camera::ViewportCamera;

pub struct UIResizeHandler;

impl Plugin for UIResizeHandler {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, ui_resize);
    }
}

fn ui_resize(
    ui_state: Res<UIState>,
    mut query_camera: Query<&mut OrthographicProjection, With<ViewportCamera>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    todo!()
}
