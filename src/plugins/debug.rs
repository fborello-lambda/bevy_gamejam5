use bevy::prelude::*;

pub(crate) struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, |query: Query<&Transform>| {
            for transform in query.iter() {
                info!("Transforms: {:?}", transform.translation);
            }
        });
    }
}
