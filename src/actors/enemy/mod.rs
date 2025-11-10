mod boid;

use crate::{ prelude::*};

pub fn plugin(app: &mut App) {
  app.add_plugins((
    // boid::plugin,
  ));
  app.add_systems(Update, spawn.in_set(AppSystems::Spawn));
}

#[derive(Component, Reflect)]
#[require(Stats)]
pub struct Enemy;

#[derive(Component, Reflect)]
pub struct Stats {
  pub vision: f32,
  pub speed: f32,
  pub attack: Attack,
  pub patrol: Patrol,
}

impl Default for Stats {
  fn default() -> Self {
    // todo!> use config loading, to avoid `tilemap::METER` mul
    Self {
      speed: 2.0,
      vision: 10.0,
      attack: default(),
      patrol: default(),
    }
  }
}

// todo!> serializable parameter
#[derive(Reflect)]
pub struct Attack {
  pub range: f32,
  pub damage: f32,
}

impl Default for Attack {
  fn default() -> Self {
    // todo!> use solid user/enemy radius or else use ray/shape casting
    Self { range: (0.5 + 0.33 + 0.25), damage: 1.0 }
  }
}

#[derive(Reflect)]
pub struct Patrol {
  pub rays: u8,
  pub range: f32,
}

impl Default for Patrol {
  fn default() -> Self {
    Self { rays: 8, range: 5.0 }
  }
}

fn spawn(
  query: Query<(Entity, &Enemy), Added<Enemy>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut commands: Commands,
) {
  let radius = 0.33;

  for (entity, _enemy) in query.iter() {
    let mesh = meshes.add(Circle::new(radius));
    let material = materials.add(Color::srgb(0.3, 0.2, 0.1));

    commands.entity(entity).insert((
      physics::enemy(),
      Controller::default(),
      Collider::circle(radius - physics::COLLIDER_OFFSET),
      YSort::default(),
      Mesh2d(mesh),
      MeshMaterial2d(material),
    ));
  }
}
