use bevy::prelude::Resource;

use crate::ui::core::state::OccupiedScreenSpace;

#[derive(Default, Resource)]
pub struct ViewportState {
    last_updated_ui_occupied_screen_space: OccupiedScreenSpace,
}

#[allow(dead_code)]
impl ViewportState {
    pub fn last_updated_ui_occupied_screen_space(&self) -> &OccupiedScreenSpace {
        &self.last_updated_ui_occupied_screen_space
    }

    pub fn last_updated_ui_occupied_screen_space_mut(&mut self) -> &mut OccupiedScreenSpace {
        &mut self.last_updated_ui_occupied_screen_space
    }

    pub fn set_last_updated_ui_occupied_screen_space(
        &mut self,
        last_updated_ui_occupied_screen_space: OccupiedScreenSpace,
    ) {
        self.last_updated_ui_occupied_screen_space = last_updated_ui_occupied_screen_space;
    }
}
