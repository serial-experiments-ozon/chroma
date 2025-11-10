use crate::prelude::*;

pub fn plugin(app: &mut App) {
  app.register_type::<D>().init_resource::<D>();
}

pub fn dev() -> bool {
  cfg!(feature = "dev")
}

#[derive(
  Debug, Resource, Reflect, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd,
)]
#[non_exhaustive]
pub enum D {
  #[default]
  None = 0,
  L1 = 1,
  L2 = 2,
}

pub fn in_debug(level: D) -> impl FnMut(Option<Res<D>>) -> bool + Clone {
  move |debug: Option<Res<D>>| {
    level <= debug.as_deref().copied().unwrap_or(D::None)
  }
}
