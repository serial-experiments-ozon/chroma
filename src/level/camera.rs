use crate::{actors::Player, prelude::*};

pub fn plugin(app: &mut App) {
  app.add_systems(Update, camera_fit_current_level);
}

// todo! must be dynamic
const ASPECT_RATIO: f32 = 16. / 9.;

#[allow(clippy::type_complexity)]
pub fn camera_fit_current_level(
  camera: Single<(&mut Projection, &mut Transform), Without<Player>>,
  player: Single<&Transform, With<Player>>,
  levels: Query<
    (&Transform, &LevelIid),
    (Without<Projection>, Without<Player>),
  >,
  ldtk: Single<&LdtkProjectHandle>,
  selection: Res<LevelSelection>,
  ldtk_assets: Res<Assets<LdtkProject>>,
) -> Result {
  // Bail early if the player isn't spawned.
  let &Transform { translation: player_translation, .. } = player.into_inner();

  let (mut projection, mut camera_transform) = camera.into_inner();
  let Projection::Orthographic(orthographic) = &mut *projection else {
    return Err(BevyError::from("non-orthographic projection found")); // unbelievable
  };

  for (level_transform, level_iid) in &levels {
    let ldtk_project = ldtk_assets
      .get(*ldtk)
      .expect("Project should be loaded if level has spawned");
    let level = ldtk_project
      .get_raw_level_by_iid(&level_iid.to_string())
      .expect("Spawned level should exist in LDtk project");

    if selection.is_match(&LevelIndices::default(), level) {
      let level_ratio = level.px_wid as f32 / level.px_hei as f32;
      orthographic.viewport_origin = Vec2::ZERO;
      if level_ratio > ASPECT_RATIO {
        // level is wider than the screen
        let height = (level.px_hei as f32 / 9.).round() * 9.;
        let width = height * ASPECT_RATIO;
        orthographic.scaling_mode =
          bevy::camera::ScalingMode::Fixed { width, height };
        camera_transform.translation.x =
          (player_translation.x - level_transform.translation.x - width / 2.)
            .clamp(0., level.px_wid as f32 - width);
        camera_transform.translation.y = 0.;
      } else {
        // level is taller than the screen
        let width = (level.px_wid as f32 / 16.).round() * 16.;
        let height = width / ASPECT_RATIO;
        orthographic.scaling_mode =
          bevy::camera::ScalingMode::Fixed { width, height };
        camera_transform.translation.y =
          (player_translation.y - level_transform.translation.y - height / 2.)
            .clamp(0., level.px_hei as f32 - height);
        camera_transform.translation.x = 0.;
      }

      camera_transform.translation.x += level_transform.translation.x;
      camera_transform.translation.y += level_transform.translation.y;
    }
  }
  Ok(())
}
