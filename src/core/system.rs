use bevy::{
  asset::AssetMetaCheck,
  log::{DEFAULT_FILTER, LogPlugin},
  prelude::*,
  window::WindowResolution,
};

pub fn plugin(app: &mut App) {
  app
    .add_plugins(
      DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
          primary_window: Window {
            resolution: WindowResolution::new(1920, 1080),
            // todo!> make configurable to use core as lib
            title: "Chromatic Echo".to_string(),
            fit_canvas_to_parent: true,
            ..default()
          }
          .into(),
          ..default()
        })
        .set(LogPlugin {
          // todo!> improve message about bevy_hanabi errors
          filter: format!("{DEFAULT_FILTER},bevy_hanabi=error"),
          ..default()
        })
        .set(AssetPlugin {
          // Wasm builds will check for meta files (that don't exist) if this isn't set.
          // This causes errors and even panics on web build on itch.
          // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
          meta_check: AssetMetaCheck::Never,
          ..default()
        }),
    )
    .insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.4)));
}
