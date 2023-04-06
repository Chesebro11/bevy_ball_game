// Check imports
use bevy::prelude::*;

use crate::game::ui::pause_menu::components::*;
// use crate::game::ui::pause_menu::styles::*;




pub fn spawn_pause_menu (
    mut commands: Commands, asset_server: Res<AssetServer>) {
        let pause_menu_entity = build_pause_menu(&mut commands, &asset_server);
}

pub fn despawn_pause_menu (
    mut commands: Commands, asset_server: Res<AssetServer>,
    pause_menu_query: Query<Entity, With<PauseMenu>>)
 {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
    commands.entity(pause_menu_entity).despawn_recursive();
    }
 }

 pub fn build_pause_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    // Whole Bunch of stuff will go here
    let pause_menu_entity = commands
        .spawn(
            (
                NodeBundle {
                    // Extract later into a const or dont idk
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
                        ..Style::DEFAULT
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                },
                PauseMenu {}
            ))
            .with_children(|parent| {
                parent.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    "Paused Game",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 32.0,
                                        color: Color::WHITE,
                                    },
                                )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                })
                .id();
            pause_menu_entity
}