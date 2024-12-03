use core::InitializeViewport;

use bevy::app::Plugin;
use update::UpdateViewportPlugin;

pub mod core;
pub mod update;

pub struct BlackThornViewportPlugin;

impl Plugin for BlackThornViewportPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(InitializeViewport)
        .add_plugins(UpdateViewportPlugin);
    }
}
