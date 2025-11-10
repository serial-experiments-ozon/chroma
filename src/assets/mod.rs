use crate::prelude::*;

#[derive(AssetCollection, Resource, Reflect)]
pub struct InteractionAssets {
  #[asset(path = "audio/sounds/button_click.ogg")]
  pub click: Handle<AudioSource>,
  #[asset(path = "audio/sounds/button_hover.ogg")]
  pub hover: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource, Reflect)]
pub struct CreditsAssets {
  #[asset(path = "audio/music/Monkeys Spinning Monkeys.ogg")]
  pub music: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource, Reflect)]
pub struct LevelAssets {
  #[asset(path = "audio/music/Fluffing A Duck.ogg")]
  pub music: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource, Reflect)]
pub struct StepsAssets {
  #[asset(path = "audio/sounds/steps/tiles", collection(typed))]
  pub tiles: Vec<Handle<AudioSource>>,
}
