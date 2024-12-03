use core::InitializeApp;

use bevy::prelude::*;
use bevy::window::PresentMode;

use ui::BlackThornUIPlugin;
use viewport::BlackThornViewportPlugin;

mod core;
mod ui;
mod viewport;

const NAME: &str = "BlackThorn";

/// The main plugin.
#[must_use]
pub struct BlackThornPlugin;

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
        .add_plugins(InitializeApp)
        .add_plugins(BlackThornUIPlugin)
        .add_plugins(BlackThornViewportPlugin);
    }
}
