use bevy::prelude::Resource;

use crate::viewport::core::table::columns::DataType;

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

    table_name: String,
    show_table_name_form: bool,

    selected_table_to_add_columnn: String,
    selected_column_data_type: DataType,
    selected_column_name: String,
    selected_column_is_nullable: bool,
    selected_column_is_pk: bool,
    show_table_column_form: bool,
}

#[allow(dead_code)]
impl UIState {
    pub fn occupied_space(&self) -> &OccupiedScreenSpace {
        &self.occupied_space
    }

    pub fn occupied_space_mut(&mut self) -> &mut OccupiedScreenSpace {
        &mut self.occupied_space
    }

    pub fn set_occupied_space(&mut self, occupied_spcae: OccupiedScreenSpace) {
        self.occupied_space = occupied_spcae;
    }

    pub fn show_table_name_form(&self) -> bool {
        self.show_table_name_form
    }

    pub fn show_table_name_form_mut(&mut self) -> &mut bool {
        &mut self.show_table_name_form
    }

    pub fn set_show_table_name_form(&mut self, show_table_name_form: bool) {
        self.show_table_name_form = show_table_name_form;
    }

    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    pub fn table_name_mut(&mut self) -> &mut String {
        &mut self.table_name
    }

    pub fn set_table_name(&mut self, table_name: String) {
        self.table_name = table_name;
    }

    pub fn show_table_column_form(&self) -> bool {
        self.show_table_column_form
    }

    pub fn show_table_column_form_mut(&mut self) -> &mut bool {
        &mut self.show_table_column_form
    }

    pub fn set_show_table_column_form(&mut self, show_table_column_form: bool) {
        self.show_table_column_form = show_table_column_form;
    }

    pub fn selected_column_data_type(&self) -> &DataType {
        &self.selected_column_data_type
    }

    pub fn selected_column_data_type_mut(&mut self) -> &mut DataType {
        &mut self.selected_column_data_type
    }

    pub fn set_selected_column_data_type(&mut self, selected_column: DataType) {
        self.selected_column_data_type = selected_column;
    }

    pub fn selected_column_is_nullable(&self) -> bool {
        self.selected_column_is_nullable
    }

    pub fn selected_column_is_nullable_mut(&mut self) -> &mut bool {
        &mut self.selected_column_is_nullable
    }

    pub fn set_selected_column_is_nullable(&mut self, selected_column_is_nullable: bool) {
        self.selected_column_is_nullable = selected_column_is_nullable;
    }

    pub fn selected_column_is_pk(&self) -> bool {
        self.selected_column_is_pk
    }

    pub fn selected_column_is_pk_mut(&mut self) -> &mut bool {
        &mut self.selected_column_is_pk
    }

    pub fn set_selected_column_is_pk(&mut self, selected_column_is_pk: bool) {
        self.selected_column_is_pk = selected_column_is_pk;
    }

    pub fn selected_table_to_add_columnn(&self) -> &str {
        &self.selected_table_to_add_columnn
    }

    pub fn selected_table_to_add_columnn_mut(&mut self) -> &mut String {
        &mut self.selected_table_to_add_columnn
    }

    pub fn set_selected_table_to_add_columnn(&mut self, selected_table_to_add_columnn: String) {
        self.selected_table_to_add_columnn = selected_table_to_add_columnn;
    }

    pub fn selected_column_name(&self) -> &str {
        &self.selected_column_name
    }

    pub fn selected_column_name_mut(&mut self) -> &mut String {
        &mut self.selected_column_name
    }

    pub fn set_selected_column_name(&mut self, selected_column_name: String) {
        self.selected_column_name = selected_column_name;
    }
}
