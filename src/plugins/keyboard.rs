use bevy::prelude::*;

use crate::components::velocity::Velocity;

pub(crate) struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_input);
    }
}

fn keyboard_input(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Velocity>>,
) {
    for mut transform in query.iter_mut() {
        if input.pressed(KeyCode::KeyW) {
            transform.translation.y += 0.1;
        }
        if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= 0.1;
        }
        if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= 0.1;
        }
        if input.pressed(KeyCode::KeyD) {
            transform.translation.x += 0.1;
        }
        if input.pressed(KeyCode::Space) {
            transform.translation = Vec3::ZERO;
        }
    }
}
