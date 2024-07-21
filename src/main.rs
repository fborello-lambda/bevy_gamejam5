use bevy::prelude::*;
use bevy::{dev_tools::fps_overlay::*, render::alpha};

mod plugins;
use plugins::{debug::*, entity::*, keyboard::*, movement::*};

mod components;

//FpsOverlayPlugin {
//    config: FpsOverlayConfig {
//        text_config: TextStyle {
//            font: default(),
//            font_size: 20.0,
//            color: Color::linear_rgb(255.0, 0.0, 0.0),
//        },
//    },
//}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::linear_rgba(0.0, 2.0, 10.0, 0.2)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 5000.,
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins(EntityPlugin)
        //.add_plugins(TranslationPlugin)
        .add_plugins(KeyboardPlugin)
        //.add_plugins(DebugPlugin)
        .run();
}

// z |
//   |___ x
//   o y
fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
