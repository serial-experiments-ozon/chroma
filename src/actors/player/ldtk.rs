use crate::{
  actors::Player,
  level::{GroundDetection, ldtk::ColliderBundle},
  prelude::*,
};

pub fn plugin(app: &mut App) {
  app.register_ldtk_entity::<PlayerBundle>("Player");
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
  pub sprite: Sprite,
  #[from_entity_instance]
  pub collider_bundle: ColliderBundle,
  pub player: Player,
  #[worldly]
  pub worldly: Worldly,
  // pub climber: Climber,
  pub ground: GroundDetection,

  // The whole EntityInstance can be stored directly as an EntityInstance component
  #[from_entity_instance]
  entity_instance: EntityInstance,
}
