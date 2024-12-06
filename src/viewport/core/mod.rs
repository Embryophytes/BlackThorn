use bevy::prelude::*;

use bevy::app::Plugin;
use bevy::app::Startup;
use camera::ViewportCamera;
use config::ViewportConfig;
use state::ViewportState;

pub mod camera;
pub mod config;
pub mod draggable;
pub mod state;
pub mod table;

pub struct InitializeViewport;

impl Plugin for InitializeViewport {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ViewportConfig::default())
            .insert_resource(ViewportState::default())
            .add_systems(Startup, initialize_camera);
    }
}

fn initialize_camera(mut commands: Commands) {
    commands.spawn((Camera2d, ViewportCamera));
}
