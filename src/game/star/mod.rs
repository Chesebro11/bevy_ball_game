use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use crate::AppState;

use resources::*;
use systems::*;

use super::SimulationState;

pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_startup_system(spawn_stars)
            .add_systems(
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            );
        // Exit State System?
        // Verify through Jacques Repository if implemented correctly
    }
}
