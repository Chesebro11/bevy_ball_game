pub mod components;
mod styles;

use bevy::prelude::*;

use systems::layout::*;
use systems::interactions::*;

use crate::AppState;
use crate::SimulationState;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app:&mut App) {
        app.add_system(spawn_pause_menu.in_schedule(OnEnter(SimulationState::Paused)))
        // .add_systems((

        // ))
    }
}
