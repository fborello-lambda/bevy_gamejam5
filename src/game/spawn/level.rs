//! Spawn the main level by triggering other observers.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::screen::Screen;

use super::player::SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);

    app.add_systems(
        Update,
        go_to_previous_level
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Digit1))),
    );

    app.add_systems(
        Update,
        go_to_next_level
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Digit2))),
    );
}

#[derive(Resource)]
pub struct Level(pub u8);

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands, level: Res<Level>) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    commands.trigger(SpawnPlayer);
    println!("Spawning level {}", level.0);
}

fn go_to_next_level(mut next_screen: ResMut<NextState<Screen>>, mut level: ResMut<Level>) {
    level.0 += 1;
    next_screen.set(Screen::Playing);
}

fn go_to_previous_level(mut next_screen: ResMut<NextState<Screen>>, mut level: ResMut<Level>) {
    if level.0 > 1 {
        level.0 -= 1;
    }
    next_screen.set(Screen::Playing);
}
