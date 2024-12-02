use bevy::prelude::Resource;

#[derive(Default)]
pub struct OccupiedScreenSpace {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

impl OccupiedScreenSpace {
    pub fn left(&self) -> f32 {
        self.left
    }

    pub fn set_left(&mut self, left: f32) {
        self.left = left;
    }

    pub fn top(&self) -> f32 {
        self.top
    }

    pub fn set_top(&mut self, top: f32) {
        self.top = top;
    }

    pub fn right(&self) -> f32 {
        self.right
    }

    pub fn set_right(&mut self, right: f32) {
        self.right = right;
    }

    pub fn bottom(&self) -> f32 {
        self.bottom
    }

    pub fn set_bottom(&mut self, bottom: f32) {
        self.bottom = bottom;
    }
}

#[derive(Default, Resource)]
pub struct UIState {
    occupied_space: OccupiedScreenSpace,
}

impl UIState {
    pub fn occupied_space(&self) -> &OccupiedScreenSpace {
        &self.occupied_space
    }

    pub fn occupied_space_mut(&mut self) -> &mut OccupiedScreenSpace {
        &mut self.occupied_space
    }

    #[allow(dead_code)]
    pub fn set_occupied_space(&mut self, occupied_spcae: OccupiedScreenSpace) {
        self.occupied_space = occupied_spcae;
    }
}
