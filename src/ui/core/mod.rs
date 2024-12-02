#[macro_export]
macro_rules! menu_button {
    (
        $ui:ident,
        $label:literal,
        $action:block
        $(, $shortcut:expr)?
    ) => {
        if $ui.add(egui::Button::new($label)$(.shortcut_text($shortcut))?).clicked()
        {
            $action
            $ui.close_menu();
        }
    };

    (
        $ui:ident,
        $enabled:ident,
        $label:literal,
        $action:block
        $(, $shortcut:expr)?
    ) => {
        if $ui.add_enabled($enabled, egui::Button::new($label)$(.shortcut_text($shortcut))?).clicked()
        {
            $action
            $ui.close_menu();
        }
    };
}

#[macro_export]
macro_rules! submenu {
    (
        $ui:ident,
        $label:literal,
        $((
            $($enabled:ident, )?
            $tag:literal,
            $($shortcut:expr, )?
            $action:block
        )),
    +) => {
        egui::menu::menu_button($ui, $label, |ui| {
            ui.set_min_width(200f32);
            let spacing = ui.spacing_mut();
            spacing.button_padding = [6f32; 2].into();
            spacing.item_spacing = [2f32; 2].into();
            ui.visuals_mut().menu_rounding = 0f32.into();

            $(
                menu_button!(ui, $($enabled, )? $tag, $action $(, $shortcut)?);
            )+
        })
        .response
        .hovered();
    };
}
