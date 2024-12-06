use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct TableColumnsComponent {
    pub columns: Vec<TableColumn>,
}

#[derive(Default, Debug)]
pub struct TableColumn {
    pub name: String,
    pub data_type: DataType,
    pub primary_key: bool,
    pub nullable: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum DataType {
    #[default]
    None,
    Integer,
    Float,
    String,
    Boolean,
    Date,
}
