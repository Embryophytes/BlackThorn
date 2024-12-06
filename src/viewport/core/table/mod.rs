use bevy::prelude::Component;

use bevy::math::Vec2;
use name::TableNameComponent;
use select::SelectComponent;

pub mod name;
pub mod select;

#[derive(Component)]
#[require(SelectComponent, TableNameComponent)]
pub struct TableComponent {
    pub size: Vec2,
}
