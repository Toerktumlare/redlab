use bevy::prelude::*;

use crate::redstone::TickCounter;

pub fn tick_the_counter(mut tick_counter: ResMut<TickCounter>) {
    if tick_counter.is_running() {
        tick_counter.tick();
    }
}
