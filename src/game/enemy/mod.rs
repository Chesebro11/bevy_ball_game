pub mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;

use bevy::prelude::*;

use super::SimulationState;

pub const ENEMY_SIZE: f32 = 64.0; // The enemy sprite is 64x64 pixels.
pub const ENEMY_SPEED: f32 = 200.0;
pub const NUMBER_OF_ENEMIES: usize = 4;
// consider impl system set like players to this
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // resource
        app.init_resource::<EnemySpawnTimer>()
            // startup_system
            // Spawn enemies when the AppState is Game
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
            // systems constantly running IF appstate and simulation state meet those conditions
            .add_systems(
                (
                    enemy_movement,
                    update_enemy_direction,
                    confine_enemy_movement,
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // Exit State Systems
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}
