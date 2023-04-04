use bevy::prelude::*;

pub mod events;
pub mod systems;
pub mod game;
pub mod mainmenu;

// use events::*;
use game::GamePlugin;
use mainmenu::MainMenuPlugin;
use systems::*;

// add_system is used when a system is running on a tick rate
// add_startup_system is used when a system is only ran on startup
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        // .add_event::<GameOver>()
        .add_plugin(GamePlugin)
        .add_plugin(MainMenuPlugin)
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]

pub enum AppState {
    MainMenu,
    #[default]
    Game,
    GameOver,
}
