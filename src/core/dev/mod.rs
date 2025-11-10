mod physics;
mod picking;
mod ui;

use bevy::{dev_tools::states::log_transitions, prelude::*};

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
  app.add_systems(Update, log_transitions::<Game>);

  app.add_plugins((physics::plugin, picking::plugin, ui::plugin));
}
