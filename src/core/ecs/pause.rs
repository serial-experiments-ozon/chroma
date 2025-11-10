use crate::prelude::*;

pub fn plugin(app: &mut App) {
  app
    .init_state::<Pause>()
    .configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));
}

/// Whether the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PausableSystems;
