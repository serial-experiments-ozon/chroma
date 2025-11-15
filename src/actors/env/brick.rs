use crate::{level::Obstacle, prelude::*};

pub fn plugin(app: &mut App) {
  app.add_systems(Update, spawn.in_set(Systems::Spawn));
}

#[derive(Component)]
#[require(Obstacle)]
pub struct Brick;

fn spawn(
  query: Query<(Entity, &Brick), Added<Brick>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut commands: Commands,
) {
  for (entity, _brick) in query.iter() {
    let mesh = meshes.add(Rectangle::new(10.0, 10.0));
    let material = materials.add(Color::srgb(0.0, 0.0, 0.0));

    commands.entity(entity).insert((
      physics::env(),
      RigidBody::Static,
      Collider::rectangle(10.0, 10.0),
      // todo!> maybe create custom sprite component?
      YSort::default(),
      Mesh2d(mesh),
      MeshMaterial2d(material),
    ));
  }
}
