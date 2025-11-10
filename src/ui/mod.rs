mod menus;
mod screens;
pub mod theme;

use crate::prelude::*;

pub use {
  menus::Menu,
  screens::Game,
  theme::{interaction, palette, widget},
};

pub fn plugin(app: &mut App) {
  app.add_plugins((menus::plugin, theme::plugin, screens::plugin));
}
