use crate::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum Transform2DSystems {
  /// Propagates changes in `Transform2D` to [`Transform`]
  Propagate,
}

pub fn plugin(app: &mut App) {
  app.configure_sets(
    PostUpdate,
    Transform2DSystems::Propagate
      .after(TransformSystems::Propagate),
  );
  app.register_type::<Transform2D>();
  app.add_systems(First, spawn);
  app.add_systems(
    PostUpdate,
    (
      sync_3d.before(TransformSystems::Propagate),
      sync_2d.in_set(Transform2DSystems::Propagate),
    ),
  );
}

#[derive(Component)]
struct Reprojection(Transform);

fn spawn(
  query: Query<(Entity, &Transform), Added<Transform>>,
  mut commands: Commands,
) {
  for (entity, &transform) in query.iter() {
    commands.entity(entity).insert(Transform2D::from(transform));
  }
}

fn sync_2d(query: Query<(Entity, &Transform2D)>, mut commands: Commands) {
  for (entity, &transform) in query.iter() {
    let transform = Transform::from(transform);
    commands.entity(entity).insert(transform).insert(Reprojection(transform));
  }
}

fn sync_3d(
  mut query: Query<(&mut Transform2D, Option<&Transform>, &Reprojection)>,
) {
  for (mut master, slave, &Reprojection(proj)) in query.iter_mut() {
    if let Some(&slave) = slave
      && slave != proj
    {
      *master = Transform2D::from(slave);
    }
  }
}

#[derive(Debug, PartialEq, Clone, Copy, Component, Reflect)]
#[reflect(Component, Default, PartialEq, Debug)]
pub struct Transform2D {
  pub translation: Vec2,
  pub rotation: Rot2,
  pub scale: Vec2,
  pub layer: f32, // Z
}

impl Transform2D {
  pub const IDENTITY: Self = Transform2D {
    translation: Vec2::ZERO,
    rotation: Rot2::IDENTITY,
    scale: Vec2::ONE,
    layer: 0.0,
  };

  pub const fn from_translation(translation: Vec2) -> Self {
    Self { translation, ..Self::IDENTITY }
  }

  pub const fn from_xy(x: f32, y: f32) -> Self {
    Self::from_translation(Vec2::new(x, y))
  }

  pub const fn layer(layer: f32) -> Self {
    Self { layer, ..Self::IDENTITY }
  }

  #[must_use]
  pub const fn with_scale(mut self, scale: Vec2) -> Self {
    self.scale = scale;
    self
  }

  #[must_use]
  pub const fn with_layer(mut self, layer: f32) -> Self {
    self.layer = layer;
    self
  }

  #[must_use]
  pub const fn add_layer(mut self, layer: f32) -> Self {
    self.layer += layer;
    self
  }

  pub fn rotate(&mut self, rotation: Rot2) {
    self.rotation = rotation * self.rotation;
  }

  pub fn rotate_z(&mut self, angle: f32) {
    self.rotate(Rot2::radians(angle));
  }

  pub fn up(&self) -> Vec2 {
    self.rotation * Vec2::Y
  }

  pub fn translation(&self) -> Vec2 {
    self.translation
  }
}

impl Default for Transform2D {
  fn default() -> Self {
    Self::IDENTITY
  }
}

impl From<Transform> for Transform2D {
  fn from(Transform { translation, rotation, scale }: Transform) -> Self {
    Self {
      translation: translation.xy(),
      rotation: Rot2::radians(rotation.to_euler(EulerRot::XYZ).2),
      scale: scale.xy(),
      layer: translation.z,
    }
  }
}

impl From<Transform2D> for Transform {
  fn from(
    Transform2D { translation, rotation, scale, layer }: Transform2D,
  ) -> Self {
    Self {
      translation: translation.extend(layer),
      rotation: Quat::from_rotation_z(rotation.as_radians()),
      scale: scale.extend(1.0),
    }
  }
}

#[test]
fn up() {
  let transform = Transform2D { rotation: Rot2::degrees(90.0), ..default() };

  assert!(Vec2::new(1.0, 0.0).angle_to(transform.up()) < f32::EPSILON);
}
