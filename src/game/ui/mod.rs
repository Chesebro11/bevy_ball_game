mod pause_menu;

use bevy::prelude::*;

use pause_menu::PauseMenuPlugin;
// use systems::interactions::*;

// use crate::AppState;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PauseMenuPlugin);
    }
}