use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_score);
    app.register_type::<Score>();
    app.insert_resource(ResScore { value: 0 });
}

#[derive(Event, Debug)]
pub struct SpawnScoreText;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Score;

#[derive(Resource)]
pub struct ResScore {
    pub value: u32,
}

fn spawn_score(_trigger: Trigger<SpawnScoreText>, mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: default(),
                    font_size: 30.0,
                    color: Color::linear_rgb(1.0, 0.0, 0.0),
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font: default(),
                    font_size: 30.0,
                    color: Color::linear_rgb(1.0, 0.0, 0.0),
                },
            ),
        ])
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            right: Val::Px(5.0),
            ..default()
        }),
        Score,
    ));
}
