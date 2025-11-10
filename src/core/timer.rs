use {
  crate::prelude::*,
  bevy::{ecs::component::Mutable, reflect::GetTypeRegistration},
};

pub trait RegisterTimer {
  fn register_timer<T: LazyTimer>(&mut self) -> &mut Self;
}

impl RegisterTimer for App {
  fn register_timer<T: LazyTimer>(&mut self) -> &mut Self {
    self.register_type::<T>();
    self.add_systems(
      Update,
      tick::<T>.in_set(PausableSystems).in_set(AppSystems::TickTimers),
    );
    self
  }
}

fn tick<T: LazyTimer>(mut query: Query<&mut T>, time: Res<Time>) {
  for mut timer in query.iter_mut() {
    timer.tick(time.delta());
  }
}

/// Simple background timer
pub trait LazyTimer:
  Component<Mutability = Mutable> + Reflect + GetTypeRegistration
{
  fn tick(&mut self, delta: Duration);

  // todo!> add message about `finished`
  fn just_finished(&self) -> bool;

  fn duration(&self) -> Duration;

  fn set_duration(&mut self, time: Duration);
}

// todo!> make derive macro or any better way
macro_rules! background_timer {
  ($ident:ident) => {
    #[derive(Component, Reflect, Clone)]
    #[reflect(Component)]
    pub struct $ident(pub Timer);

    impl $ident {
      pub fn new(secs: f32) -> Self {
        Self(Timer::from_seconds(secs, TimerMode::Repeating))
      }
    }

    impl LazyTimer for $ident {
      fn tick(&mut self, delta: Duration) {
        self.0.tick(delta);
      }

      fn just_finished(&self) -> bool {
        self.0.just_finished()
      }

      fn duration(&self) -> Duration {
        self.0.duration()
      }

      fn set_duration(&mut self, duration: Duration) {
        self.0.set_duration(duration)
      }
    }
  };
}

pub(crate) use background_timer;
