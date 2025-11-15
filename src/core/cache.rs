use bevy::{ecs::query::QueryFilter, prelude::*};

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Cached<T>(T);

pub fn store<T: Component + Clone, F: QueryFilter>(
  query: Query<(Entity, &T), F>,
  mut commands: Commands,
) {
  for (entity, component) in query.iter() {
    commands.entity(entity).insert(Cached(component.clone()));
  }
}

pub fn restore<T: Component + Clone, F: QueryFilter>(
  query: Query<(Entity, &Cached<T>), (With<T>, F)>,
  mut commands: Commands,
) {
  for (entity, cached) in query.iter() {
    commands.entity(entity).insert(cached.0.clone()).remove::<Cached<T>>();
  }
}
