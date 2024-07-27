use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    // Print state transitions in dev builds
    app.add_plugins(FpsOverlayPlugin {
        config: FpsOverlayConfig {
            text_config: TextStyle {
                font: default(),
                font_size: 20.0,
                color: Color::linear_rgb(255.0, 0.0, 0.0),
            },
        },
    });
}
