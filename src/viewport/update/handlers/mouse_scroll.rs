use bevy::input::mouse::MouseScrollUnit;
use bevy::prelude::*;

use bevy::app::Plugin;
use bevy::app::Update;
use bevy::input::mouse::MouseWheel;
use bevy::window::PrimaryWindow;

use crate::ui::core::state::UIState;
use crate::viewport::core::camera::ViewportCamera;

pub struct MouseScrollHandler;

impl Plugin for MouseScrollHandler {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, mouse_wheel_scroll);
    }
}

fn mouse_wheel_scroll(
    mut mouse_wheel_event_reader: EventReader<MouseWheel>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    ui_state: Res<UIState>,
    mut viewport_camera_projection_query: Query<&mut OrthographicProjection, With<ViewportCamera>>,
) {
    let cursor_position = match primary_window_query.single().cursor_position() {
        Some(position) => position,
        None => return,
    };

    let window_width = primary_window_query.single().width();
    let window_height = primary_window_query.single().height();

    let ui_occupied_screen_space = ui_state.occupied_space();

    if cursor_position.x < ui_occupied_screen_space.left()
        || cursor_position.x > window_width - ui_occupied_screen_space.right()
    {
        return;
    }

    if cursor_position.y < ui_occupied_screen_space.top()
        || cursor_position.y > window_height - ui_occupied_screen_space.top()
    {
        return;
    }

    let mut projection = match viewport_camera_projection_query.get_single_mut() {
        Ok(projection) => projection,
        _ => unreachable!(),
    };

    for event in mouse_wheel_event_reader.read() {
        match event.unit {
            MouseScrollUnit::Line => {
                projection.scale += event.y;
            }
            MouseScrollUnit::Pixel => todo!(),
        }
    }
}
