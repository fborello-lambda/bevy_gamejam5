//! Spawn the main level by triggering other observers.

use std::ops::Deref;

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

#[derive(Resource, Default, Debug, PartialEq, PartialOrd, Eq)]
pub enum Level{
    #[default]
    Birth,
    Ocean,
    BackToLake,
    Death
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands, level: Res<Level>) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    commands.trigger(SpawnPlayer);
    println!("Spawning level {:#?}", level);
}

fn go_to_next_level(mut next_screen: ResMut<NextState<Screen>>, mut level: ResMut<Level>) {
    *level = next_level_from_enum(level.deref());
    next_screen.set(Screen::Playing);
}

fn go_to_previous_level(mut next_screen: ResMut<NextState<Screen>>, mut level: ResMut<Level>) {
    *level = prev_level_from_enum(level.deref());
    next_screen.set(Screen::Playing);
}

fn next_level_from_enum(level: &Level) -> Level {
    match *level {
        Level::Birth => Level::Ocean,
        Level::Ocean => Level::BackToLake,
        Level::BackToLake => Level::Death,
        Level::Death => Level::Birth
    }
}

fn prev_level_from_enum(level: &Level) -> Level {
    match *level {
        Level::Birth => Level::Death,
        Level::Ocean => Level::Birth,
        Level::BackToLake => Level::Ocean,
        Level::Death => Level::BackToLake
    }
}
