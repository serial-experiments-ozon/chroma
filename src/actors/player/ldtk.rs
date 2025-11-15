use crate::{
  actors::Player,
  level::{GroundDetector, ldtk::ColliderBundle},
  prelude::*,
};

pub fn plugin(app: &mut App) {
  app.register_ldtk_entity::<PlayerBundle>("Player");
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
  pub player: Player,
  // #[from_entity_instance]
  // pub collider_bundle: ColliderBundle,
  #[worldly]
  pub worldly: Worldly,
  // pub climber: Climber,

  // The whole EntityInstance can be stored directly as an EntityInstance component
  #[from_entity_instance]
  entity_instance: EntityInstance,
}
