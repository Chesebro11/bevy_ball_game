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

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Configure System Sets
            .configure_set(MovementSystemSet.before(ConfinementSystemSet))
            // On enter State
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    player_movement.in_set(MovementSystemSet),
                    confine_player_movement.in_set(ConfinementSystemSet),
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_systems(
                (enemy_hit_player, player_hit_star)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // Exit State
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
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

// Doing the same the but in an enum
// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// pub enum PlayerSystemSet {
//     Movement,
//     Confinement
// }
// How this would look within the plugin: