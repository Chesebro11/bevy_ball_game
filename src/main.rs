use bevy::prelude::*;

pub mod events;
pub mod systems;

use crate::enemy::EnemyPlugin;
use crate::player::PlayerPlugin;
use crate::score::ScorePlugin;
use crate::star::StarPlugin;

mod enemy;
mod player;
mod score;
mod star;

use events::*;
use systems::*;

// add_system is used when a system is running on a tick rate
// add_startup_system is used when a system is only ran on startup
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GameOver>()
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(StarPlugin)
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}
