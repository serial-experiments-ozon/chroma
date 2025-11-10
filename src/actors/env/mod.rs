mod brick;

use crate::prelude::*;

pub use brick::Brick;

pub fn plugin(app: &mut App) {
  app.add_plugins(brick::plugin);
}
