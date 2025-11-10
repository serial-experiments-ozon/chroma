mod assets;
mod ldtk;
mod state;

use crate::prelude::*;

pub use state::{Action, Walk};

background_timer!(StepsTimer);

pub fn plugin(app: &mut App) {
  app
    .register_type::<Stats>()
    .register_type::<Player>()
    .register_timer::<StepsTimer>()
    .add_plugins((assets::plugin, state::plugin, ldtk::plugin))
    .add_systems(
      Update,
      (spawn, steps.run_if(in_state(Game::Gameplay))).in_set(AppSystems::Spawn),
    );
}

#[derive(Component, Reflect, Default, Clone)]
#[require(Stats)]
pub struct Player;

#[derive(Component, Reflect)]
pub struct Stats {
  pub speed: f32,
}

impl Default for Stats {
  fn default() -> Self {
    Self { speed: 3.6 }
  }
}

fn spawn(
  query: Query<(Entity, &Player), Added<Player>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut commands: Commands,
) {
  let radius = 10.0;

  for (entity, _player) in query.iter() {
    let mesh = meshes.add(Circle::new(radius));
    let material = materials.add(Color::srgb(0.1, 0.2, 0.1));

    commands
      .entity(entity)
      .insert((
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::circle(radius - physics::COLLIDER_OFFSET),
        YSort::default(),
        Mesh2d(mesh),
        MeshMaterial2d(material),
      ))
      .insert((state::bundle(), StepsTimer::new(TIME_BETWEEN_STEPS)));
  }
}

const TIME_BETWEEN_STEPS: f32 = 0.3;

fn steps(
  mut commands: Commands,
  steps: Res<StepsAssets>,
  query: Query<(Entity, &StepsTimer), (With<Player>, With<Walk>)>,
) {
  let mut rng = rand::rng();
  for (entity, timer) in query.iter() {
    if timer.just_finished()
      && let Some(effect) = steps.tiles.choose(&mut rng).cloned()
    {
      let half = PlaybackSettings::ONCE.with_volume(Volume::Linear(0.15));
      commands.entity(entity).with_child(sound_effect_with(effect, half));
    }
  }
}
