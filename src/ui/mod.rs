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
use crate::viewport::core::draggable::DraggableComponent;
use crate::viewport::core::table::columns::DataType;
use crate::viewport::core::table::columns::TableColumn;
use crate::viewport::core::table::columns::TableColumnsComponent;
use crate::viewport::core::table::name::TableNameComponent;
use crate::viewport::core::table::TableComponent;

pub mod core;

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
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut table_names_columns_component_query: Query<(&TableNameComponent, &mut TableColumnsComponent), With<TableComponent>>,
) {
    let ctx = contexts.ctx_mut();

    let occupied_screen_space_top = TopBottomPanel::top("top_panel")
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
                add_submenu!(
                    ui,
                    "Tools",
                    ("Add table", "Ctrl + N", {
                        ui_state.set_show_table_name_form(true);
                    }),
                    ("Add column", "Ctrl + M", {
                        ui_state.set_show_table_column_form(true);
                    })
                );
            });
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();

    let occupied_screen_space_left = egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();

    let occupied_screen_space_right = egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();

    let occupied_screen_space_bottom = egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();

    let occupied_screen_space = ui_state.occupied_space_mut();
    occupied_screen_space.set_left(occupied_screen_space_left);
    occupied_screen_space.set_right(occupied_screen_space_right);
    occupied_screen_space.set_top(occupied_screen_space_top);
    occupied_screen_space.set_bottom(occupied_screen_space_bottom);

    if ui_state.show_table_name_form() {
        egui::Window::new("Table name")
            .vscroll(true)
            .auto_sized()
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.text_edit_singleline(ui_state.table_name_mut());
                ui.add_space(10.0f32);
                if ui.button("Ok").clicked() {
                    if !(ui_state.table_name() == "") {
                        let table_size = Vec2::new(80., 100.);
                        let shape = meshes.add(Rectangle::new(table_size.x, table_size.y));
                        commands.spawn((
                            Mesh2d(shape),
                            MeshMaterial2d(materials.add(Color::srgb(0.2f32, 0.3f32, 0.3f32))),
                            Transform::from_xyz(0.0, 0.0, 0.0),
                            TableComponent { size: table_size },
                            DraggableComponent,
                            TableNameComponent {
                                name: ui_state.table_name().to_string(),
                            },
                        ));
    
                        ui_state.set_table_name("".to_string());
                        ui_state.set_show_table_name_form(false);
                    }
                }
            });
    }

    if ui_state.show_table_column_form() {
        egui::Window::new("Table columns")
            .vscroll(true)
            .auto_sized()
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                egui::ComboBox::from_label("Select table!")
                    .selected_text(format!("{:?}", ui_state.selected_table_to_add_columnn()))
                    .show_ui(ui, |ui| {
                        for (name, _) in table_names_columns_component_query.iter() {
                            ui.selectable_value(ui_state.selected_table_to_add_columnn_mut(), name.name.clone(), name.name.clone());
                        }
                    });
                ui.text_edit_singleline(ui_state.selected_column_name_mut());
                egui::ComboBox::from_label("Select data type!")
                    .selected_text(format!("{:?}", ui_state.selected_column_data_type()))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(ui_state.selected_column_data_type_mut(), DataType::Boolean, "Boolean");
                        ui.selectable_value(ui_state.selected_column_data_type_mut(), DataType::Date, "Date");
                        ui.selectable_value(ui_state.selected_column_data_type_mut(), DataType::Float, "Float");
                        ui.selectable_value(ui_state.selected_column_data_type_mut(), DataType::Integer, "Integer");
                        ui.selectable_value(ui_state.selected_column_data_type_mut(), DataType::String, "String");
                    });
                ui.checkbox(ui_state.selected_column_is_nullable_mut(), "Nullable");
                ui.checkbox(ui_state.selected_column_is_pk_mut(), "A part of PK");
                if ui.button("Ok").clicked() {
                    if !(ui_state.selected_table_to_add_columnn() == ""
                        || ui_state.selected_column_name() == ""
                        || *ui_state.selected_column_data_type() == DataType::None
                        || (ui_state.selected_column_is_pk() && ui_state.selected_column_is_nullable()))
                    {
                        for (name, mut columns) in table_names_columns_component_query.iter_mut() {
                            if name.name == ui_state.selected_table_to_add_columnn() {
                                columns.columns.push(TableColumn {
                                    name: ui_state.selected_column_name().to_string(),
                                    data_type: ui_state.selected_column_data_type().clone(),
                                    primary_key: ui_state.selected_column_is_pk(),
                                    nullable: ui_state.selected_column_is_nullable(),
                                });
                            }
                        }
    
                        ui_state.set_selected_table_to_add_columnn("".to_string());
                        ui_state.set_selected_column_name("".to_string());
                        ui_state.set_selected_column_data_type(DataType::None);
                        ui_state.set_selected_column_is_nullable(false);
                        ui_state.set_selected_column_is_pk(false);
                        ui_state.set_show_table_column_form(false);
                    }
                }
            });
    }


    for (name, columns) in table_names_columns_component_query.iter() {
        info!("{:?}, {:#?}", name.name, columns.columns);
    }
}
