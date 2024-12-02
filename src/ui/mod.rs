use bevy::app::AppExit;
use bevy::app::Plugin;
use bevy::app::Update;

use bevy::prelude::*;

use bevy::window::PrimaryWindow;
use bevy_egui::egui;
use bevy_egui::egui::TopBottomPanel;
use bevy_egui::EguiContexts;
use bevy_egui::EguiPlugin;

use crate::menu_button;
use crate::submenu;

use crate::OccupiedScreenSpace;
use crate::OriginalCameraTransform;

mod core;

const MENU_BAR_HEIGHT: f32 = 34f32;
const CAMERA_TARGET: Vec3 = Vec3::ZERO;

#[must_use]
#[derive(Default)]
pub struct BlackThornUIPlugin {}

impl Plugin for BlackThornUIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Update, egui_system)
            .add_systems(Update, update_camera_transform_system);
    }
}

fn egui_system(
    mut contexts: EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
    mut exit: EventWriter<AppExit>,
) {
    let ctx = contexts.ctx_mut();

    occupied_screen_space.top = TopBottomPanel::top("top_panel")
        .exact_height(MENU_BAR_HEIGHT)
        .show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let spacing = ui.spacing_mut();
                spacing.button_padding = [6f32; 2].into();
                spacing.item_spacing = [2f32; 2].into();
                ui.visuals_mut().menu_rounding = 0f32.into();

                submenu!(
                    ui,
                    "File",
                    ("Quit", "Ctrl + Q", {
                        exit.send(AppExit::Success);
                    })
                );
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
