use {super::Stats, crate::prelude::*};

pub fn plugin(app: &mut App) {
  app.add_systems(FixedUpdate, boid.in_set(ControllerInputsSet));
}

#[derive(Component, Reflect)]
pub struct Boid(pub f32);

fn boid(
  mut controllers: Query<&mut Controller>,
  query: Query<(Entity, &Transform2D, &Stats, &Boid)>,
) {
  for [(entity, a, stats, &Boid(force)), (_, b, _, _)] in
    query.iter_combinations()
  {
    let movement = a.translation - b.translation;
    let affect = -(movement.length() - stats.vision) / stats.vision;

    if affect > 0.0
      && let Ok(mut controller) = controllers.get_mut(entity)
    {
      controller
        .control(Control::Move(movement.normalize_or_zero() * affect * force));
    }
  }
}
