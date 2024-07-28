//! Handles spawning of entities. Here, we are using
//! [observers](https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Observer.html)
//! for this, but you could also use `Events<E>` or `Commands`.

use bevy::prelude::*;

pub mod level;
pub mod player;
pub mod food;
pub mod level1_score;

pub(super) fn plugin(app: &mut App) {
    // This is not the best way, we should separate the logic of each level
    app.add_plugins((level::plugin, player::plugin, food::plugin, level1_score::plugin));
}
