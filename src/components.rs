use bevy::prelude::*;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    // This will allow the enemy to keep track of movement
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Star {}