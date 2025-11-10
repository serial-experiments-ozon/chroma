use crate::prelude::*;

pub fn plugin(app: &mut App) {
  app.add_systems(
    Update,
    toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
  );
}

const TOGGLE_KEY: KeyCode = KeyCode::F1;

fn toggle_debug_ui(
  mut options: ResMut<UiDebugOptions>,
  mut debug: ResMut<D>,
  mut local: Local<Option<D>>,
) {
  options.toggle();

  if let Some(d) = local.take() {
    *local = Some(*debug);
    *debug = d;
  } else {
    *local = Some(D::L2);
  }
}
