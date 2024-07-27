//! Spawn the player.

use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    game::{
        animation::Animations,
        movement::{Movement, MovementController, WrapWithinWindow},
    },
    screen::Screen,
};

use super::level::{self, Level};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

const INITIAL_POSITION: Vec3 = Vec3::ZERO;

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    level: Res<Level>,
) {
    let mut scene_bundle = SceneBundle {
        transform: Transform::from_translation(INITIAL_POSITION),
        scene: asset_server.load("Salmon.glb#Scene0"),
        ..default()
    };
    scene_bundle.transform.rotate_y(PI / 2.0);

    let mut graph = AnimationGraph::new();
    let animations = graph
        .add_clips(
            [asset_server.load("Salmon.glb#Animation0")],
            1.0,
            graph.root,
        )
        .collect();

    let graph_handle = graphs.add(graph);
    commands.insert_resource(Animations {
        graph: graph_handle,
        animations,
    });
    let velocity = if *level != Level::BackToLake {
        Vec3::new(0.0, 0.0, 0.0)
    } else {
        Vec3::new(20.0, 0.0, 0.0)
    };
    commands.spawn((
        Name::new("Player"),
        Player,
        scene_bundle,
        MovementController::default(),
        Movement {
            velocity,
            acceleration: Vec3::ZERO,
        },
        WrapWithinWindow,
        StateScoped(Screen::Playing),
    ));
}
