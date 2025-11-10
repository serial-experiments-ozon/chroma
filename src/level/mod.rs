mod camera;

use crate::{
  actors::{Enemy, Player, enemy},
  prelude::*,
};

pub fn plugin(app: &mut App) {
  app.register_type::<LevelAssets>();
  app.configure_loading_state(
    LoadingStateConfig::new(Game::Loading)
      // -- assets --
      .load_collection::<LevelAssets>(),
  );

  app.add_plugins((camera::plugin));
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

pub fn spawn_level(mut commands: Commands, level_assets: Res<LevelAssets>) {
  commands
    .spawn((Name::new("Level"), DespawnOnExit(Game::Gameplay), Level {}))
    .insert(children![
      (Name::new("Player"), Player),
      // (Name::new("Gameplay Music"), music(level_assets.music.clone()))
    ]);
}
