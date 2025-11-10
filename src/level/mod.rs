mod camera;
mod ground;
pub mod ldtk;

use crate::{
  actors::{Enemy, Player, enemy},
  prelude::*,
};

pub use ground::GroundDetection;

pub fn plugin(app: &mut App) {
  app.register_type::<LevelAssets>();
  app.configure_loading_state(
    LoadingStateConfig::new(Game::Loading)
      // -- assets --
      .load_collection::<LevelAssets>(),
  );

  app.add_plugins((camera::plugin, ldtk::plugin, ground::plugin));
}

// todo!> find better name
#[derive(Component, Default, Copy, Clone)]
#[require(Obstacle)]
pub struct Difficulty;

#[derive(Component, Default, Copy, Clone)]
pub struct Obstacle;

#[derive(Component)]
#[require(Visibility, Transform)]
pub struct Level {}

pub fn spawn_level(mut commands: Commands, assets: Res<LevelAssets>) {
  let ldtk_handle = LdtkProjectHandle::from(assets.level.clone());
  commands
    .spawn((Name::new("Level"), DespawnOnExit(Game::Gameplay), Level {}))
    .insert(LdtkWorldBundle { ldtk_handle, ..Default::default() })
    .with_children(|parent| {
      // parent.spawn((Name::new("Player"), Player));
    });
}
