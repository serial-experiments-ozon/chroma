mod pause;
mod transform;

use crate::prelude::*;

pub use {
    pause::{PausableSystems, Pause},
    transform::{Transform2D, Transform2DSystems},
};

pub fn plugin(app: &mut App) {
  app.add_plugins((pause::plugin, transform::plugin));
}
