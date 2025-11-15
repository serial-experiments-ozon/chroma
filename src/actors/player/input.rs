use crate::prelude::*;

pub fn plugin(app: &mut App) {
  app.add_plugins(InputManagerPlugin::<Action>::default());
}

#[derive(Actionlike, Reflect, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
  #[actionlike(Axis)]
  Move,
  Jump,
  Walk,
}

pub fn map() -> InputMap<Action> {
  InputMap::default()
    .with_axis(Action::Move, VirtualAxis::ad())
    .with(Action::Jump, KeyCode::Space)
    .with(Action::Walk, KeyCode::ShiftLeft)
}
