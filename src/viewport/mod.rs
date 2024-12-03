use core::InitializeViewport;

use bevy::app::Plugin;

pub mod camera;
pub mod core;

pub struct BlackThornViewportPlugin;

impl Plugin for BlackThornViewportPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(InitializeViewport);
    }
}
