use bevy::prelude::*;

use crate::components::velocity::*;

pub(crate) struct TranslationPlugin;
impl Plugin for TranslationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
        //app.add_systems(Update, update_velocity);
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.v3 * time.delta_seconds();
    }
}

fn update_velocity(time: Res<Time>, mut query: Query<&mut Velocity>) {
    for mut velocity in query.iter_mut() {
        velocity.v3 *= time.delta_seconds();
    }
}
