use crate::prelude::*;

pub fn plugin(app: &mut App) {
  app.add_systems(Update, (spawn_sensor, detection, update));
}

#[derive(Component)]
pub struct GroundSensor {
  pub entity: Entity,
  pub intersection: HashSet<Entity>,
}

#[derive(Clone, Default, Component)]
pub struct GroundDetection {
  pub on_ground: bool,
}

pub fn spawn_sensor(
  mut commands: Commands,
  detect_ground_for: Query<(Entity, &Collider), Added<GroundDetection>>,
) {
  for (entity, collider) in &detect_ground_for {
    if let Some(cuboid) = collider.shape().as_cuboid() {
      // Avoid directly `nalgebra::Vector` usage
      let (x, y) = (cuboid.half_extents.x, cuboid.half_extents.y);

      let detector = Collider::rectangle(x, 4.); // todo! avoid constant wide
      let translation = Vec3::new(0., -y, 0.);

      commands.entity(entity).with_children(|builder| {
        builder
          .spawn(detector)
          .insert((Sensor, CollisionEventsEnabled))
          .insert((
            GlobalTransform::default(),
            Transform::from_translation(translation),
          ))
          .insert(GroundSensor { entity, intersection: HashSet::new() });
      });
    }
  }
}

pub fn detection(
  (mut start, mut end): (
    MessageReader<CollisionStart>,
    MessageReader<CollisionEnd>,
  ),
  mut ground_sensors: Query<&mut GroundSensor>,
  colliders: Query<Entity, (With<Collider>, Without<Sensor>)>,
) {
  for &CollisionStart { collider1, collider2, .. } in start.read() {
    if colliders.contains(collider1) {
      if let Ok(mut sensor) = ground_sensors.get_mut(collider2) {
        sensor.intersection.insert(collider1);
      }
    } else if colliders.contains(collider2) {
      if let Ok(mut sensor) = ground_sensors.get_mut(collider1) {
        sensor.intersection.insert(collider2);
      }
    }
  }
  for &CollisionEnd { collider1, collider2, .. } in end.read() {
    if colliders.contains(collider1) {
      if let Ok(mut sensor) = ground_sensors.get_mut(collider2) {
        sensor.intersection.remove(&collider1);
      }
    } else if colliders.contains(collider2) {
      if let Ok(mut sensor) = ground_sensors.get_mut(collider1) {
        sensor.intersection.remove(&collider2);
      }
    }
  }
}

pub fn update(
  mut detectors: Query<&mut GroundDetection>,
  sensors: Query<&GroundSensor, Changed<GroundSensor>>,
) {
  for sensor in &sensors {
    if let Ok(mut detection) = detectors.get_mut(sensor.entity) {
      // todo! maybe its better to use `On` instead or couple
      detection.on_ground = !sensor.intersection.is_empty();
    }
  }
}
