use bevy::prelude::Component;

use bevy::math::Vec2;
use select::SelectComponent;

pub mod select;

#[derive(Component)]
#[require(SelectComponent)]
pub struct TableComponent {
    pub size: Vec2,
}
