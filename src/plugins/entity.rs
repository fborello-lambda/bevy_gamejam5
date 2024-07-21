use std::f32::consts::PI;
use std::time::Duration;

use bevy::animation::animate_targets;
use bevy::prelude::*;

use crate::components::velocity::*;

const INITIAL_VELOCITY: Vec3 = Vec3::new(1.0, 0.2, 0.0);
const INITIAL_POSITION: Vec3 = Vec3::new(-20.0, 0.0, 0.0);

#[derive(Bundle, Clone)]
pub(crate) struct EntityBundle {
    pub(crate) asset: SceneBundle,
    pub(crate) velocity: Velocity,
}
pub(crate) struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_entity);
        app.add_systems(Update, setup_scene_once_loaded.before(animate_targets));
    }
}

#[derive(Resource)]
struct Animations {
    animations: Vec<AnimationNodeIndex>,
    #[allow(dead_code)]
    graph: Handle<AnimationGraph>,
}

fn spawn_entity(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let mut bundle = SceneBundle {
        transform: Transform::from_translation(INITIAL_POSITION),
        scene: asset_server.load("Salmon.glb#Scene0"),
        ..default()
    };
    bundle.transform.rotate_y(PI / 2.0);

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

    commands.spawn(EntityBundle {
        asset: bundle,
        velocity: Velocity {
            v3: INITIAL_VELOCITY,
        },
    });
}

fn setup_scene_once_loaded(
    mut commands: Commands,
    animations: Res<Animations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut players {
        let mut transitions = AnimationTransitions::new();

        // Make sure to start the animation via the `AnimationTransitions`
        // component. The `AnimationTransitions` component wants to manage all
        // the animations and will get confused if the animations are started
        // directly via the `AnimationPlayer`.
        transitions
            .play(&mut player, animations.animations[0], Duration::ZERO)
            .repeat();

        commands
            .entity(entity)
            .insert(animations.graph.clone())
            .insert(transitions);
    }
}
