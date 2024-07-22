//! Handle player input and translate it into movement.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/latest/examples/movement/physics_in_fixed_timestep.rs).

use bevy::{prelude::*, window::PrimaryWindow};

use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    // Record directional input as movement controls.
    app.register_type::<MovementController>();
    app.add_systems(
        Update,
        record_movement_controller.in_set(AppSet::RecordInput),
    );

    // Apply movement based on controls.
    app.register_type::<(Movement, WrapWithinWindow)>();
    app.add_systems(
        Update,
        (update_movement, wrap_within_window)
            .chain()
            .in_set(AppSet::Update),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MovementController{
    pub intent: Vec2,
    pub action: bool,
}

fn record_movement_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController>,
) {
    // Collect directional input.
    let mut intent = Vec2::ZERO;
    let mut action = false;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }
    if input.just_pressed(KeyCode::Space) {
        action = true;
    }

    // Normalize so that diagonal movement has the same speed as
    // horizontal and vertical movement.
    let intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.intent = intent;
        controller.action = action;
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Movement {
    pub acceleration: Vec3,
    pub velocity: Vec3,
}

fn update_movement(
    time: Res<Time>,
    mut movement_query: Query<(&MovementController, &mut Movement, &mut Transform)>,
) {
    for (controller, mut movement, mut transform) in &mut movement_query {
        let controller_acceleration = controller.intent * 50.0;
        movement.acceleration = controller_acceleration.extend(0.0);

        let acceleration = movement.acceleration;
        movement.velocity += acceleration * time.delta_seconds();

        transform.translation += movement.velocity * time.delta_seconds();

        if movement.velocity.length() > 0.0{
            let rotation = Quat::from_rotation_arc(Vec3::Z, movement.velocity.normalize());
            transform.rotation = rotation;
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WrapWithinWindow;

fn wrap_within_window(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut wrap_query: Query<&mut Transform, With<WrapWithinWindow>>,
) {
    let size = window_query.single().size() + 256.0;
    let half_size = size / 2.0;
    for mut transform in &mut wrap_query {
        let position = transform.translation.xy();
        let wrapped = (position + half_size).rem_euclid(size) - half_size;
        transform.translation = wrapped.extend(transform.translation.z);
    }
}
