use crate::prelude::*;

pub fn plugin(app: &mut App) {
  app.add_systems(
    Update,
    zoom.in_set(PausableSystems).in_set(AppSystems::RecordInput),
  );
}

fn zoom(mut proj: Single<&mut Projection, With<PrimaryCamera>>) {
  if let Projection::Orthographic(proj) = &mut **proj {}
}
