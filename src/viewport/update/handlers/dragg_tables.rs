use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use bevy::input::common_conditions::input_pressed;
use bevy::window::PrimaryWindow;

use crate::ui::core::state::UIState;
use crate::viewport::core::camera::ViewportCamera;
use crate::viewport::core::table::select::SelectComponent;
use crate::viewport::core::table::TableComponent;

pub struct DraggTablesHandler;

impl Plugin for DraggTablesHandler {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            mouse_left_button_hold.run_if(input_pressed(MouseButton::Left)),
        );
    }
}

fn mouse_left_button_hold(
    mut mouse_motion_event_reader: EventReader<MouseMotion>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    ui_state: Res<UIState>,
    mut tables_transform_mesh_query: Query<(&mut Transform, &SelectComponent), With<TableComponent>>,
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

    let projection = match viewport_camera_projection_query.get_single_mut() {
        Ok(projection) => projection,
        _ => unreachable!(),
    };

    for (mut transform, select_component) in tables_transform_mesh_query.iter_mut() {
        if select_component.selected
        {
            for event in mouse_motion_event_reader.read() {
                transform.translation +=
                    Vec3::new(event.delta.x, (-1.) * event.delta.y, 0.0f32) * projection.scale;
            }
        }
    }
}
