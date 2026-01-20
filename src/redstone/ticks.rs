use bevy::prelude::*;

use crate::redstone::{GlobalTick, Scheduler, Tick};

#[derive(Message)]
pub struct GlobalTickEvent(Tick);

pub fn tick_the_counter(
    mut tick_counter: ResMut<GlobalTick>,
    mut scheduler: ResMut<Scheduler>,
    mut writer: MessageWriter<GlobalTickEvent>,
) {
    if tick_counter.is_running() {
        tick_counter.tick();
        let now = tick_counter.read();
        scheduler.advance(now);
        writer.write(GlobalTickEvent(now));
    }
}
