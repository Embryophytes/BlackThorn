use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use bevy::input::common_conditions::input_pressed;
use bevy::window::PrimaryWindow;

use crate::ui::core::state::UIState;
use crate::viewport::core::camera::ViewportCamera;
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
    mut tables_transform_mesh_query: Query<(&mut Transform, &TableComponent)>,
    camera_global_transform_query: Query<(&Camera, &GlobalTransform), With<ViewportCamera>>,
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

    let (camera, camera_transform) = camera_global_transform_query.single();

    let window = primary_window_query.single();

    let projection = match viewport_camera_projection_query.get_single_mut() {
        Ok(projection) => projection,
        _ => unreachable!(),
    };

    if let Some(cursor_world_position) = window
        .cursor_position()
        .map(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.unwrap().origin.truncate())
    {
        for (mut transform, table_component) in tables_transform_mesh_query.iter_mut() {
            if cursor_world_position.x >= transform.translation.x - table_component.size.x / 2.
                && cursor_world_position.x <= transform.translation.x + table_component.size.x / 2.
                && cursor_world_position.y >= transform.translation.y - table_component.size.y / 2.
                && cursor_world_position.y <= transform.translation.y + table_component.size.y / 2.
            {
                for event in mouse_motion_event_reader.read() {
                    transform.translation +=
                        Vec3::new(event.delta.x, (-1.) * event.delta.y, 0.0f32) * projection.scale;
                }
            }
        }
    }
}
