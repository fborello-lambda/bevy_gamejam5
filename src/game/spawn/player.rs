//! Spawn the player.

use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    game::{
        animation::Animations,
        assets::{GlbKey, HandleMap},
        movement::{Movement, MovementController, WrapWithinWindow},
    },
    screen::Screen,
};

use super::level::Level;

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
pub const DEFAULT_SCALE: f32 = 0.3;
pub const MAX_SCALE: f32 = 1.0;


fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    asset_server: Res<HandleMap<GlbKey>>,
    animation_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    level: Res<Level>,
) {
    let mut scene_bundle = SceneBundle {
        transform: Transform::from_translation(INITIAL_POSITION),
        scene: asset_server[&GlbKey::Salmon].clone_weak(),
        ..default()
    };
    scene_bundle.transform.rotate_y(PI / 2.0);

    scene_bundle.transform = scene_bundle
        .transform
        .with_scale(Vec3::from_array([DEFAULT_SCALE; 3]));

    let mut graph = AnimationGraph::new();
    let animations = graph
        .add_clips(
            [animation_server.load("Salmon.glb#Animation0")],
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
        RigidBody::KinematicPositionBased,
        Collider::capsule_z(1.0, 1.0),
        Ccd::enabled(),
    ));
}
