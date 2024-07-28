use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::spawn::{
    food::Food,
    level1_score::{ResScore, Score},
    player::{Player, DEFAULT_SCALE, MAX_SCALE},
};
pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, collision_despawn_food_set_score);
}

fn collision_despawn_food_set_score(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut query_text: Query<&mut Text, With<Score>>,
    mut query_fish: Query<&mut Transform, With<Player>>,
    mut score: ResMut<ResScore>,
    query: Query<(Entity, &Food)>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                let check_entity1 = query.get(*entity1).is_ok();
                let check_entity2 = query.get(*entity2).is_ok();

                if check_entity1 && check_entity2 {
                    continue;
                }
                // Check if either entity is despawnable
                else {
                    if check_entity1 {
                        commands.entity(*entity1).despawn();
                    } else if check_entity2 {
                        commands.entity(*entity2).despawn();
                    }

                    let mut text = query_text.single_mut();
                    score.value += 1;
                    text.sections[1].value = format!("{}", score.value);

                    let mut transform = query_fish.single_mut();
                    *transform = transform.with_scale(Vec3::from_array(
                        [(DEFAULT_SCALE + (score.value as f32 / 10.0))
                            .clamp(DEFAULT_SCALE, MAX_SCALE); 3],
                    ));
                }
            }
            CollisionEvent::Stopped(_, _, _) => continue,
        }
    }
}
