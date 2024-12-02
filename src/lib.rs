use core::Initialize;

use bevy::prelude::*;
use bevy::window::PresentMode;

use ui::BlackThornUIPlugin;

mod core;
mod ui;

#[derive(Default, Resource)]
struct OccupiedScreenSpace {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

#[derive(Resource, Deref, DerefMut)]
struct OriginalCameraTransform(Transform);

const NAME: &str = "BlackThorn";
const CAMERA_TARGET: Vec3 = Vec3::ZERO;

/// The main plugin.
#[must_use]
#[derive(Default)]
pub struct BlackThornPlugin {}

impl Plugin for BlackThornPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let window = Window {
            title: NAME.into(),
            position: WindowPosition::At((0, 0).into()),
            resize_constraints: WindowResizeConstraints {
                min_width: 640f32,
                min_height: 480f32,
                ..Default::default()
            },
            present_mode: PresentMode::AutoNoVsync,
            ..Default::default()
        };

        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(window),
            ..Default::default()
        }))
        .add_plugins(Initialize::default())
        .add_plugins(BlackThornUIPlugin::default())
        .init_resource::<OccupiedScreenSpace>()
        .add_systems(Startup, setup_system);
    }
}

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    let camera_pos = Vec3::new(-2.0, 2.5, 5.0);
    let camera_transform =
        Transform::from_translation(camera_pos).looking_at(CAMERA_TARGET, Vec3::Y);
    commands.insert_resource(OriginalCameraTransform(camera_transform));

    commands.spawn((Camera3d::default(), camera_transform));
}
