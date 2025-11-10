use crate::prelude::*;

pub fn plugin(app: &mut App) {
  app.add_systems(PostUpdate, ysort.before(TransformSystems::Propagate));
}

pub const BACKGROUND_OFFSET: f32 = -128.0;
// todo!> next use more complex way with true background
pub const YSORT_COHESION: f32 = 0.00001;

#[derive(Component, Default)]
pub struct YSort(pub f32);

pub fn ysort(
  mut transform: Query<(&mut Transform2D, &GlobalTransform, &YSort)>,
) {
  for (mut transform, global_transform, YSort(offset)) in transform.iter_mut() {
    transform.layer =
      offset - global_transform.translation().y * YSORT_COHESION;
  }
}
