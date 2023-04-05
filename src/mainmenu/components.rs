use bevy::prelude::Component;

#[derive(Component)]

pub struct MainMenu {}

// Two buttons components that get used in systems/layout
#[derive(Component)]
pub struct PlayButton {}

#[derive(Component)]
pub struct QuitButton {}
