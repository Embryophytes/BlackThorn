use bevy::prelude::*;

use bevy::input::common_conditions::input_just_pressed;
use bevy::window::PrimaryWindow;

use crate::ui::core::state::UIState;
use crate::viewport::core::camera::ViewportCamera;
use crate::viewport::core::table::select::SelectComponent;
use crate::viewport::core::table::TableComponent;

pub struct SelectTablesHandler;

impl Plugin for SelectTablesHandler {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            mouse_left_button_press.run_if(input_just_pressed(MouseButton::Left)),
        );
    }
}

fn mouse_left_button_press(
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    ui_state: Res<UIState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut tables_transform_selection_query: Query<(
        &Transform,
        &mut SelectComponent,
        &MeshMaterial2d<ColorMaterial>,
        &TableComponent,
    )>,
    camera_global_transform_query: Query<(&Camera, &GlobalTransform), With<ViewportCamera>>,
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

    if let Some(cursor_world_position) = window
        .cursor_position()
        .map(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.unwrap().origin.truncate())
    {
        for (transform, mut select_component, color_material, table_component) in
            tables_transform_selection_query.iter_mut()
        {
            if cursor_world_position.x >= transform.translation.x - table_component.size.x / 2.
                && cursor_world_position.x <= transform.translation.x + table_component.size.x / 2.
                && cursor_world_position.y >= transform.translation.y - table_component.size.y / 2.
                && cursor_world_position.y <= transform.translation.y + table_component.size.y / 2.
            {
                select_component.selected = true;
                materials.get_mut(color_material).unwrap().color =
                    Color::srgb(0.3f32, 0.4f32, 0.5f32);
            } else {
                select_component.selected = false;
                materials.get_mut(color_material).unwrap().color =
                    Color::srgb(0.2f32, 0.3f32, 0.3f32);
            }
        }
    }
}
