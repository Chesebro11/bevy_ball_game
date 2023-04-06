pub mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use bevy::app::AppExit;
use systems::interactions::*;
use systems::layout::*;

use crate::game::SimulationState;
use crate::AppState;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_pause_menu.in_schedule(OnEnter(SimulationState::Paused)))
            // .add_system(interact_with_resume_button.in_set(OnUpdate(SimulationState::Paused)))
            // .add_system(interact_with_menu_button.in_set(OnUpdate(SimulationState::Paused)))
            .add_systems(
                (interact_with_resume_button, interact_with_menu_button)
                .in_set(OnUpdate(SimulationState::Paused)),
            )
            .add_system(despawn_pause_menu.in_schedule(OnEnter(SimulationState::Running)));
    }
}
