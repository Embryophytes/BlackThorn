use bevy::app::Plugin;
use config::AppConfig;
use state::AppState;

pub mod config;
pub mod state;

pub struct InitializeApp;

impl Plugin for InitializeApp {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(AppConfig::default())
            .insert_resource(AppState::default());
    }
}
