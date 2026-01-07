use bevy::prelude::*;

use crate::redstone::GlobalTick;

#[derive(Message)]
pub struct GlobalTickEvent;

pub fn tick_the_counter(
    mut tick_counter: ResMut<GlobalTick>,
    mut writer: MessageWriter<GlobalTickEvent>,
) {
    if tick_counter.is_running() {
        tick_counter.tick();
        writer.write(GlobalTickEvent);
    }
}
