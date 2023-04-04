use bevy::prelude::*;

pub mod components;
pub mod systems;

use crate::AppState;

use systems::*;

use super::SimulationState;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

// Doing the same the but in an enum
// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// pub enum PlayerSystemSet {
//     Movement,
//     Confinement
// }
// How this would look within the plugin:


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(MovementSystemSet.before(ConfinementSystemSet))
            .add_startup_system(spawn_player)

            .add_systems (
                (
                player_movement.in_set(MovementSystemSet),
                confine_player_movement.in_set(ConfinementSystemSet),
                )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(enemy_hit_player)
            .add_system(player_hit_star);
    } // Do I want to add a state that spawns and despawns the player upon entering and exiting game state?
}


// .add_system(System::in_set(PlayerSystemSet::Movement))

// .before allows us to guarantee the order in which some sytems operate
// .add_system(player_movement.before(confine_player_movement))
// .add_system(confine_player_movement)//.after(player_movement)
// .add_systems(
//     (
//         player_movement,
//         confine_player_movement
//     ).chain() // This is another way of using .before or .after but the nested systems allows for better readabillity.
//)
