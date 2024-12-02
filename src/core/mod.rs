use app_state::AppState;
use bevy::app::Plugin;

use config::AppConfig;

pub mod app_state;
pub mod config;

#[derive(Default)]
pub struct Initialize {}

impl Plugin for Initialize {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(AppConfig::default())
            .insert_resource(AppState::default());
    }
}
