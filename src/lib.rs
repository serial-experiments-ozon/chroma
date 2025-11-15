#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
#![feature(iter_map_windows)]

use crate::prelude::*;

mod actors;
mod core;
mod level;
mod ui;
//
mod assets;
pub mod prelude;

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(core::plugin);
    app.add_loading_state(
      LoadingState::new(Game::Loading).continue_to_state(Game::Title),
    );

    app.add_plugins(StateMachinePlugin::default());

    app.add_plugins((ui::plugin, level::plugin, actors::plugin));
    app.add_systems(Startup, spawn_camera);
  }
}

fn spawn_camera(mut commands: Commands) {
  commands.spawn((Name::new("Camera"), PrimaryCamera));
}
