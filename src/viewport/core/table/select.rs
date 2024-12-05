use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct SelectComponent {
    pub selected: bool,
}
