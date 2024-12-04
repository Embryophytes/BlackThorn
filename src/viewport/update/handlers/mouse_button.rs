use bevy::prelude::*;

use bevy::input::common_conditions::*;
use bevy::input::mouse::MouseMotion;

use bevy::app::Plugin;
use bevy::app::Update;

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
    mut viewport_camera_transform_query: Query<&mut Transform, With<ViewportCamera>>,
) {
    let mut transform = match viewport_camera_transform_query.get_single_mut() {
        Ok(transform) => transform,
        _ => unreachable!(),
    };

    for event in mouse_motion_event_reader.read() {
        transform.translation += Vec3::new((-1.) * event.delta.x, event.delta.y, 0.0f32);
    }
}
