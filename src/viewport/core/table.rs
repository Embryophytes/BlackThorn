use bevy::prelude::Component;

use bevy::math::Vec2;

#[derive(Component)]
pub struct TableComponent {
    pub size: Vec2,
}
