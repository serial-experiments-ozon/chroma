mod assets;
mod input;
mod ldtk;
mod state;

use crate::prelude::*;

pub use state::Grounded;

background_timer!(StepsTimer);

pub fn plugin(app: &mut App) {
  register(app)
    .add_plugins((assets::plugin, state::plugin, input::plugin, ldtk::plugin))
    .add_systems(
      Update,
      (spawn, steps.run_if(in_state(Game::Gameplay))).in_set(Systems::Spawn),
    );
}

fn register(app: &mut App) -> &mut App {
  app
    .register_type::<Stats>()
    .register_type::<Player>()
    .register_timer::<StepsTimer>()
}

#[derive(Component, Reflect, Default, Clone)]
#[require(Stats)]
pub struct Player;

#[derive(Component, Reflect)]
pub struct Stats {
  pub speed: f32,
}

impl Default for Stats {
  fn default() -> Self {
    Self { speed: 32.0 }
  }
}

#[derive(Component)]
pub enum WallCaster {
  Left,
  Right,
}

fn spawn(
  query: Query<(Entity, &Player), Added<Player>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut commands: Commands,
) {
  let radius = 10.0;

  for (player, _) in query.iter() {
    let mesh = meshes.add(Circle::new(radius));
    let material = materials.add(Color::srgb(0.1, 0.2, 0.1));

    commands
      .entity(player)
      .insert((input::map(), state::Controller))
      .insert((
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Mesh2d(mesh),
        MeshMaterial2d(material),
      ))
      .insert(state::MoveInfo::default())
      .insert(Collider::compound(vec![(
        Vec2::new(0.0, -2.0),
        Rotation::default(),
        Collider::rectangle(12.0, 16.0), // 12 x 16
      )]))
      .insert(CollisionLayers::new(
        Layers::PlayerCollider,
        [Layers::Terrain, Layers::Platform],
      ))
      .insert(
        ShapeCaster::new(
          Collider::rectangle(11.8, 0.5),
          Vec2::new(0., -9.75),
          0.0,
          Dir2::NEG_Y,
        )
        .with_max_distance(0.5) // todo! i hate this constant
        .with_max_hits(8)
        .with_query_filter(
          SpatialQueryFilter::default()
            .with_mask([Layers::Terrain, Layers::Platform]),
        ),
      )
      .insert(Friction {
        dynamic_coefficient: 0.0,
        static_coefficient: 0.0,
        combine_rule: CoefficientCombine::Min,
      })
      .insert(Restitution {
        coefficient: 0.0,
        combine_rule: CoefficientCombine::Min,
      })
      .insert((StepsTimer::new(TIME_BETWEEN_STEPS)));

    let shape_caster = |dir: Dir2| {
      ShapeCaster::new(
        Collider::rectangle(0.5, 10.0),
        Vec2::new(dir.x.signum() * 6.75, -1.75),
        0.0,
        dir,
      )
      .with_max_distance(0.5) // fixme! i hate this constant
      .with_max_hits(8) // i love this constant
      .with_query_filter(
        SpatialQueryFilter::default()
          .with_mask([Layers::Terrain /* ... */]),
      )
    };

    commands.entity(player).insert(children![
      (WallCaster::Left, shape_caster(Dir2::NEG_X)),
      (WallCaster::Right, shape_caster(Dir2::X))
    ]);
  }
}

const TIME_BETWEEN_STEPS: f32 = 0.5;

fn steps(
  mut commands: Commands,
  steps: Res<StepsAssets>,
  query: Query<(Entity, &StepsTimer), (With<Player>, With<Grounded>)>,
) {
  let mut rng = rand::rng();
  for (entity, timer) in query.iter() {
    if timer.just_finished()
      && let Some(effect) = steps.tiles.choose(&mut rng).cloned()
    {
      let half = PlaybackSettings::ONCE.with_volume(Volume::Linear(0.05));
      commands.entity(entity).with_child(sound_effect_with(effect, half));
    }
  }
}
