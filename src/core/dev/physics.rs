use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
  app
    .add_plugins(PhysicsDebugPlugin::default())
    .insert_gizmo_config(
      PhysicsGizmos::default(),
      GizmoConfig { enabled: false, ..default() },
    )
    .add_systems(
      Update,
      toggle_physics_debug_render.run_if(input_just_pressed(TOGGLE_KEY)),
    );

  app
    .add_plugins((PhysicsDiagnosticsPlugin, PhysicsDiagnosticsUiPlugin))
    .insert_resource(PhysicsDiagnosticsUiSettings {
      enabled: false,
      ..default()
    })
    .add_systems(
      Update,
      toggle_physics_diagnostics_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );
}

const TOGGLE_KEY: KeyCode = KeyCode::F2;

fn toggle_physics_debug_render(mut gizmos: ResMut<GizmoConfigStore>) {
  gizmos.config_mut::<PhysicsGizmos>().0.enabled ^= true;
}

fn toggle_physics_diagnostics_ui(
  mut settings: ResMut<PhysicsDiagnosticsUiSettings>,
) {
  settings.enabled ^= true;
}
