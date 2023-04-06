pub mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use systems::layout::*;
// use systems::interactions::*;

use crate::AppState;
use crate::game::SimulationState;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app:&mut App) {
        app.add_system(spawn_pause_menu.in_schedule(OnEnter(SimulationState::Paused)))
        // .add_systems((
            // app.add_system(spawn_pause_menu);
        // ))
        .add_system(despawn_pause_menu.in_schedule(OnEnter(SimulationState::Running)));
    }
}
