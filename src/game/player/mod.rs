use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

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
            .add_system(player_movement.in_set(MovementSystemSet))
            .add_system(confine_player_movement.in_set(ConfinementSystemSet))

            .add_system(enemy_hit_player)
            .add_system(player_hit_star);
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
