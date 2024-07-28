use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;
use std::ops::Range;

use crate::game::assets::{GlbKey, HandleMap};

const FOOD_RANGE_X: Range<f32> = -18.0..18.0;
const FOOD_RANGE_Y: Range<f32> = -10.0..10.0;
const FOOD_SPAWN_INTERVAL: f32 = 1.0;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(FoodSpawnRate {
        timer: Timer::from_seconds(FOOD_SPAWN_INTERVAL, TimerMode::Repeating),
    })
    .add_systems(Update, spawn_food);
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Food;

#[derive(Resource, Debug)]
pub struct FoodSpawnRate {
    timer: Timer,
}

fn spawn_food(
    mut commands: Commands,
    mut food_timer: ResMut<FoodSpawnRate>,
    query: Query<&Transform, With<Food>>,
    asset_server: Res<HandleMap<GlbKey>>,
    time: Res<Time>,
) {
    food_timer.timer.tick(time.delta());
    if !food_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    let pos = Vec3::new(
        rng.gen_range(FOOD_RANGE_X),
        rng.gen_range(FOOD_RANGE_Y),
        0.0,
    );

    let scene_bundle = SceneBundle {
        transform: Transform::from_translation(pos),
        scene: asset_server[&GlbKey::Food].clone_weak(),
        ..default()
    };

    if query.iter().size_hint().0 <= 5 {
        commands.spawn((
            scene_bundle,
            Food,
            RigidBody::KinematicPositionBased,
            Collider::ball(0.5),
            Ccd::enabled(),
            ActiveCollisionTypes::KINEMATIC_KINEMATIC,
            ActiveEvents::COLLISION_EVENTS,
        ));
    }
}
