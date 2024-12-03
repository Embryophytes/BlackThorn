use bevy::prelude::*;

use bevy::app::Update;
use bevy::app::Plugin;

use crate::ui::core::state::UIState;
use crate::viewport::core::camera::ViewportCamera;
use crate::viewport::core::state::ViewportState;

pub struct UIResizeHandler;

impl Plugin for UIResizeHandler {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, ui_resize);
    }
}

fn ui_resize(
    mut viewport_state: ResMut<ViewportState>,
    ui_state: Res<UIState>,
    mut viewport_camera_transform_query: Query<&mut Transform, With<ViewportCamera>>,
) {
    let last_updated_ui_occupied_screen_space = viewport_state.last_updated_ui_occupied_screen_space_mut();
    let occupied_screen_space = ui_state.occupied_space();

    let mut transform = match viewport_camera_transform_query.get_single_mut() {
        Ok(transform) => transform,
        _ => unreachable!(),
    };

    let left_delta = occupied_screen_space.left() - last_updated_ui_occupied_screen_space.left();
    let right_delta = occupied_screen_space.right() - last_updated_ui_occupied_screen_space.right();
    let top_delta = occupied_screen_space.top() - last_updated_ui_occupied_screen_space.top();
    let bottom_delta = occupied_screen_space.bottom() - last_updated_ui_occupied_screen_space.bottom();

    let x_delta = (right_delta - left_delta) / 2.;
    let y_delta = (top_delta - bottom_delta) / 2.;

    debug!("Updating viewport, (x_delta, y_delta) = ({}, {})", x_delta, y_delta);

    // let left_taken = occupied_screen_space.left() / window.width();
    // let right_taken = occupied_screen_space.right() / window.width();
    // let top_taken = occupied_screen_space.top() / window.height();
    // let bottom_taken = occupied_screen_space.bottom() / window.height();
    
    transform.translation += Vec3::new(x_delta, y_delta, 0.0f32);

    last_updated_ui_occupied_screen_space.set_left(occupied_screen_space.left());
    last_updated_ui_occupied_screen_space.set_right(occupied_screen_space.right());
    last_updated_ui_occupied_screen_space.set_top(occupied_screen_space.top());
    last_updated_ui_occupied_screen_space.set_bottom(occupied_screen_space.bottom());
}
