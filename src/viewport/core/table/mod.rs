use bevy::prelude::Component;

use bevy::math::Vec2;
use columns::TableColumnsComponent;
use name::TableNameComponent;
use select::SelectComponent;

pub mod columns;
pub mod name;
pub mod select;

#[derive(Component)]
#[require(SelectComponent, TableNameComponent, TableColumnsComponent)]
pub struct TableComponent {
    pub size: Vec2,
}
