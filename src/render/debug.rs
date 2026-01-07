use bevy::prelude::*;

use crate::{TickText, redstone::GlobalTick};

pub fn render_debug_info(
    tick_counter: Res<GlobalTick>,
    mut query: Query<&mut TextSpan, With<TickText>>,
) {
    for mut span in &mut query {
        **span = format!("{}", tick_counter.read());
    }
}
