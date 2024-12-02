use bevy::prelude::*;
use bevy::window::PresentMode;

use bevy::window::PrimaryWindow;
use bevy_egui::egui;
use bevy_egui::EguiContexts;
use bevy_egui::EguiPlugin;

#[derive(Default, Resource)]
struct OccupiedScreenSpace {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

const CAMERA_TARGET: Vec3 = Vec3::ZERO;

#[derive(Resource, Deref, DerefMut)]
struct OriginalCameraTransform(Transform);

const NAME: &str = "BlackThorn";

const MENU_BAR_HEIGHT: f32 = 34f32;

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
        .add_plugins(EguiPlugin)
        .init_resource::<OccupiedScreenSpace>()
        .add_systems(Startup, setup_system)
        .add_systems(Update, egui_system)
        .add_systems(Update, update_camera_transform_system);
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

fn egui_system(
    mut contexts: EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
    mut exit: EventWriter<AppExit>,
) {
    let ctx = contexts.ctx_mut();

    occupied_screen_space.top = egui::TopBottomPanel::top("top_panel")
        .exact_height(MENU_BAR_HEIGHT)
        .show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let spacing = ui.spacing_mut();
                spacing.button_padding = [6f32; 2].into();
                spacing.item_spacing = [2f32; 2].into();
                ui.visuals_mut().menu_rounding = 0f32.into();

                egui::menu::menu_button(ui, "File", |ui| {
                    ui.set_min_width(200f32);
                    let spacing = ui.spacing_mut();
                    spacing.button_padding = [6f32; 2].into();
                    spacing.item_spacing = [2f32; 2].into();
                    ui.visuals_mut().menu_rounding = 0f32.into();

                    if ui
                        .add(egui::Button::new("Quit").shortcut_text("Ctrl + Q"))
                        .clicked()
                    {
                        exit.send(AppExit::Success);
                        ui.close_menu();
                    }
                })
                .response
                .hovered();
            });
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();

    occupied_screen_space.left = egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();

    occupied_screen_space.right = egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();

    occupied_screen_space.bottom = egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();
}

fn update_camera_transform_system(
    occupied_screen_space: Res<OccupiedScreenSpace>,
    original_camera_transform: Res<OriginalCameraTransform>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&Projection, &mut Transform)>,
) {
    let (camera_projection, mut transform) = match camera_query.get_single_mut() {
        Ok((Projection::Perspective(projection), transform)) => (projection, transform),
        _ => unreachable!(),
    };

    let distance_to_target = (CAMERA_TARGET - original_camera_transform.translation).length();
    let frustum_height = 2.0 * distance_to_target * (camera_projection.fov * 0.5).tan();
    let frustum_width = frustum_height * camera_projection.aspect_ratio;

    let window = windows.single();

    let left_taken = occupied_screen_space.left / window.width();
    let right_taken = occupied_screen_space.right / window.width();
    let top_taken = occupied_screen_space.top / window.height();
    let bottom_taken = occupied_screen_space.bottom / window.height();
    transform.translation = original_camera_transform.translation
        + transform.rotation.mul_vec3(Vec3::new(
            (right_taken - left_taken) * frustum_width * 0.5,
            (top_taken - bottom_taken) * frustum_height * 0.5,
            0.0,
        ));
}
