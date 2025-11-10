use crate::{
  prelude::*,
  ui::{Game, widget},
};

pub(super) fn plugin(app: &mut App) {
  app.add_systems(OnEnter(Game::Loading), spawn_loading_screen);
}

fn spawn_loading_screen(mut commands: Commands) {
  commands.spawn((
    widget::ui_root("Loading Screen"),
    DespawnOnExit(Game::Loading),
    children![widget::label("Loading...")],
  ));
}
