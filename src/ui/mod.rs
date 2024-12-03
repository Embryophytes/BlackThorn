use core::state::UIState;
use core::InitializeUI;

use bevy::app::AppExit;
use bevy::app::Plugin;
use bevy::app::Update;

use bevy::prelude::*;

use bevy_egui::egui;
use bevy_egui::egui::TopBottomPanel;
use bevy_egui::EguiContexts;
use bevy_egui::EguiPlugin;

use crate::add_menu_button;
use crate::add_submenu;

mod core;

const MENU_BAR_HEIGHT: f32 = 34f32;

#[must_use]
pub struct BlackThornUIPlugin;

impl Plugin for BlackThornUIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(EguiPlugin)
            .add_plugins(InitializeUI)
            .add_systems(Update, egui_system);
    }
}

fn egui_system(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UIState>,
    mut exit: EventWriter<AppExit>,
) {
    let ctx = contexts.ctx_mut();

    let occupied_screen_space = ui_state.occupied_space_mut();

    occupied_screen_space.set_top(
        TopBottomPanel::top("top_panel")
            .exact_height(MENU_BAR_HEIGHT)
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    let spacing = ui.spacing_mut();
                    spacing.button_padding = [6f32; 2].into();
                    spacing.item_spacing = [2f32; 2].into();
                    ui.visuals_mut().menu_rounding = 0f32.into();

                    add_submenu!(
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
            .height(),
    );

    occupied_screen_space.set_left(
        egui::SidePanel::left("left_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            })
            .response
            .rect
            .width(),
    );

    occupied_screen_space.set_right(
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            })
            .response
            .rect
            .width(),
    );

    occupied_screen_space.set_bottom(
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            })
            .response
            .rect
            .height(),
    );
}
