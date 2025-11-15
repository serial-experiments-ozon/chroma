use {
  crate::{actors::Player, prelude::*},
  avian2d::math::*,
};

use super::input::Action as Input;

/// The number of [`FixedUpdate`] steps the player can jump for after pressing the spacebar.
const SHOULD_JUMP_TICKS: isize = 8;
/// The number of [`FixedUpdate`] steps the player can jump for after falling off an edge.
const COYOTE_TIME_TICKS: isize = 5;
/// The number of [`FixedUpdate`] steps the player should receive upward velocity for.
const JUMP_BOOST_TICKS: isize = 2;

/// Max player horizontal velocity.
const PLAYER_MAX_H_VEL: f32 = 1.5;
/// Max player vertical velocity.
const PLAYER_MAX_Y_VEL: f32 = 5.;
/// The positive y velocity added to the player every jump boost tick.
const PLAYER_JUMP_VEL: f32 = 2.2;
/// The x velocity added to the player when A/D is held.
const PLAYER_MOVE_VEL: f32 = 0.4;
/// The y velocity subtracted from the player due to gravity.
const PLAYER_GRAVITY: f32 = 0.15;

pub fn plugin(app: &mut App) {
  register(app)
    .add_systems(
      FixedUpdate,
      (update_grounded, movement).chain().in_set(Systems::Update),
    )
    .add_systems(Update, keyboard_input.in_set(Systems::Input));
  // restore velocity after pause
  app
    .add_systems(OnExit(Game::Gameplay), cache::store::<LinearVelocity, ()>)
    .add_systems(OnEnter(Game::Gameplay), cache::restore::<LinearVelocity, ()>);
}

fn register(app: &mut App) -> &mut App {
  app.add_message::<Action>()
}

/// A [`Message`] written for a movement input action.
#[derive(Message, Debug, Copy, Clone)]
pub enum Action {
  Move(Scalar),
  Jump,
  JumpCut,
  Walk(bool),
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component)]
pub struct Controller;

/// A marker component indicating that an entity is on the ground.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

/// A bundle that contains components for character movement.
#[derive(Component, Default)]
pub struct MoveInfo {
  pub should_jump_ticks: isize,
  pub coyote_time_ticks: isize,
  pub jump_boost_ticks: isize,
  pub walk: bool, // just relax
}

pub fn keyboard_input(
  player: Single<&ActionState<Input>>,
  mut events: MessageWriter<Action>,
) {
  let state = player.into_inner();

  let direction = state.clamped_value(&Input::Move);

  if direction.abs() > 0.1 {
    // todo! fix hardcoded dead-zone (feat:gamepad)
    events.write(Action::Move(direction));
  }

  if state.just_pressed(&Input::Jump) {
    events.write(Action::Jump);
  }
  if state.just_released(&Input::Jump) {
    events.write(Action::JumpCut);
  }

  if state.just_pressed(&Input::Walk) {
    events.write(Action::Walk(true));
  }
  if state.just_released(&Input::Walk) {
    events.write(Action::Walk(false));
  }
}

pub fn update_grounded(
  mut commands: Commands,
  mut query: Query<(Entity, &ShapeHits), With<Controller>>,
) {
  for (entity, hits) in &mut query {
    let is_grounded = hits.iter().next().is_some();
    if is_grounded {
      commands.entity(entity).insert(Grounded);
    } else {
      commands.entity(entity).remove::<Grounded>();
    }
  }
}

pub fn movement(
  mut events: MessageReader<Action>,
  player: Single<
    (&mut MoveInfo, &mut LinearVelocity, &ShapeHits, Has<Grounded>),
    With<Player>,
  >,
  wall_casters: Query<(&ShapeHits, &super::WallCaster), Without<Player>>,
) {
  let (mut info, mut velocity, shape_hits, is_grounded) = player.into_inner();

  if is_grounded {
    info.coyote_time_ticks = COYOTE_TIME_TICKS;
  }

  let mut moved = false;
  for action in events.read().copied() {
    match action {
      Action::Move(direction) => {
        velocity.x += direction * PLAYER_MOVE_VEL * 64.;
        moved = true;
      }
      Action::Jump => {
        info.should_jump_ticks = SHOULD_JUMP_TICKS;
      }
      Action::JumpCut => {
        if velocity.y > 0. {
          velocity.y /= 3.;
          info.jump_boost_ticks = 0;
          info.should_jump_ticks = 0;
        }
      }
      Action::Walk(walk) => info.walk = walk,
    }
  }

  if info.should_jump_ticks > 0 && info.coyote_time_ticks > 0 {
    info.jump_boost_ticks = JUMP_BOOST_TICKS;
  }

  let too_close = shape_hits.iter().any(|hit| hit.distance < 0.25);
  if info.jump_boost_ticks > 0 {
    velocity.y = PLAYER_JUMP_VEL * 64.;
  } else if too_close && velocity.y < 0.5 {
    velocity.y = 0.45;
  } else if is_grounded && velocity.y < 0.5 {
    velocity.y = 0.;
  } else {
    velocity.y -= PLAYER_GRAVITY * 64.;
  }

  velocity.y =
    velocity.y.clamp(-PLAYER_MAX_Y_VEL * 64., PLAYER_MAX_Y_VEL * 64.);

  if !moved {
    velocity.x *= 0.6;
    if velocity.x.abs() < 0.1 {
      velocity.x = 0.;
    }
  }

  for (wall_hits, side) in wall_casters.iter() {
    let too_close = wall_hits.iter().any(|hit| hit.distance < 0.25);
    let any_hit = wall_hits.iter().next().is_some();

    match side {
      super::WallCaster::Left => {
        if too_close && velocity.x < 0.5 {
          velocity.x = 0.45;
        } else if any_hit && velocity.x < 0.5 {
          velocity.x = 0.;
        }
      }
      super::WallCaster::Right => {
        if too_close && velocity.x > -0.5 {
          velocity.x = -0.45;
        } else if any_hit && velocity.x > -0.5 {
          velocity.x = 0.;
        }
      }
    }
  }

  let walk_modifier = if info.walk { 0.5 } else { 1.0 }; // todo! hidden constant

  velocity.x = velocity.x.clamp(
    -PLAYER_MAX_H_VEL * 64. * walk_modifier,
    PLAYER_MAX_H_VEL * 64. * walk_modifier,
  );

  // todo! maybe use `MoveInfo`
  info.should_jump_ticks -= 1;
  info.jump_boost_ticks -= 1;
  info.coyote_time_ticks -= 1;
}
