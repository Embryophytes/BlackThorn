use bevy::prelude::*;

use bevy::input::common_conditions::*;
use bevy::input::mouse::MouseMotion;

use bevy::app::Plugin;
use bevy::app::Update;
use bevy::window::PrimaryWindow;

use crate::ui::core::state::UIState;
use crate::viewport::core::camera::ViewportCamera;

pub struct MouseButtonHandler;

impl Plugin for MouseButtonHandler {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            mouse_right_button_hold.run_if(input_pressed(MouseButton::Right)),
        );
    }
}

fn mouse_right_button_hold(
    mut mouse_motion_event_reader: EventReader<MouseMotion>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    ui_state: Res<UIState>,
    viewport_camera_projection_query: Query<&mut OrthographicProjection, With<ViewportCamera>>,
    mut viewport_camera_transform_query: Query<&mut Transform, With<ViewportCamera>>,
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

    let mut transform = match viewport_camera_transform_query.get_single_mut() {
        Ok(transform) => transform,
        _ => unreachable!(),
    };

    let projection = match viewport_camera_projection_query.get_single() {
        Ok(projection) => projection,
        _ => unreachable!(),
    };

    for event in mouse_motion_event_reader.read() {
        transform.translation += Vec3::new((-1.) * event.delta.x, event.delta.y, 0.0f32) * projection.scale;
    }
}
