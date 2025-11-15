mod collider;
mod walls;

use crate::{actors::Player, prelude::*};

pub use collider::{ColliderBundle, SensorBundle};

pub fn plugin(app: &mut App) {
  app
    .add_plugins(LdtkPlugin)
    .insert_resource(LevelSelection::Uid(0))
    .insert_resource(LdtkSettings {
      // Use the world translation directly for level spawns
      level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
        load_level_neighbors: true,
      },
      set_clear_color: SetClearColor::FromLevelBackground,
      ..default()
    })
    .add_systems(Update, update_level_selection);

  app.add_plugins(walls::plugin);
}

fn update_level_selection(
  levels: Query<(&LevelIid, &Transform), Without<Player>>,
  players: Query<&Transform, With<Player>>,
  mut selection: ResMut<LevelSelection>,
  ldtk: Single<&LdtkProjectHandle>,
  ldtk_assets: Res<Assets<LdtkProject>>,
) {
  for (level_iid, Transform { translation, .. }) in &levels {
    if let Some(ldtk) = ldtk_assets.get(*ldtk) {
      let level = ldtk
        .get_raw_level_by_iid(&level_iid.to_string())
        .expect("Spawned level should exist in LDtk project");

      let bounds = Rect {
        min: Vec2::new(translation.x, translation.y),
        max: Vec2::new(
          translation.x + level.px_wid as f32,
          translation.y + level.px_hei as f32,
        ),
      };

      for Transform { translation, .. } in &players {
        if translation.x < bounds.max.x
          && translation.x > bounds.min.x
          && translation.y < bounds.max.y
          && translation.y > bounds.min.y
          && !selection.is_match(&LevelIndices::default(), level)
        {
          *selection = LevelSelection::iid(level.iid.clone());
        }
      }
    }
  }
}
