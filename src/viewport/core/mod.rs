use bevy::prelude::*;

use bevy::app::Plugin;
use bevy::app::Startup;
use camera::ViewportCamera;
use config::ViewportConfig;
use state::ViewportState;
use table::TableComponent;

pub mod camera;
pub mod config;
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

fn initialize_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2d, ViewportCamera));

    let shape = meshes.add(Rectangle::new(80.0, 100.0));

    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(Color::srgb(0.3f32, 0.4f32, 0.5f32))),
        Transform::from_xyz(
            -40.0,
            0.0,
            0.0,
        ),
        TableComponent,
    ));
}
