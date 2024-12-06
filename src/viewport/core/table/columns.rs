use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct TableColumnsComponent {
    pub columns: Vec<TableColumn>,
}

//TODO: Remone lint allow
#[allow(dead_code)]
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

impl From<DataType> for thorn_root::schema::data_type::DataType {
    fn from(value: DataType) -> Self {
        match value {
            DataType::None => unreachable!(),
            DataType::Integer => thorn_root::schema::data_type::DataType::Integer,
            DataType::Float => thorn_root::schema::data_type::DataType::Float,
            DataType::String => thorn_root::schema::data_type::DataType::String,
            DataType::Boolean => thorn_root::schema::data_type::DataType::Boolean,
            DataType::Date => thorn_root::schema::data_type::DataType::Date,
        }
    }
}
