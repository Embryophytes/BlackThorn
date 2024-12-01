use bevy::app::App;

use blackthorn::BlackThornPlugin;

fn main() {
    App::new().add_plugins(BlackThornPlugin::default()).run();
}
