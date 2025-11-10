use crate::{prelude::*, ui::Game};

pub fn plugin(app: &mut App) {
  app.configure_loading_state(
    LoadingStateConfig::new(Game::Loading).load_collection::<StepsAssets>(),
  );
}

