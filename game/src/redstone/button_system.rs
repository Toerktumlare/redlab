use bevy::prelude::*;

use crate::{
    BlockType,
    grid_plugin::{BlockChange, BlockChangeQueue, Grid, UpdateRequest},
    redstone::ticks::GlobalTickEvent,
    render::{Position, Pressed},
};

pub fn button_tick_system(
    mut reader: MessageReader<GlobalTickEvent>,
    mut queue: ResMut<BlockChangeQueue>,
    grid: Res<Grid>,
    query: Query<&Position, With<Pressed>>,
) {
    for _ in reader.read() {
        for pos in query.iter() {
            let Some(block_data) = grid.get(pos.0) else {
                continue;
            };

            let BlockType::StoneButton {
                pressed,
                attached_face,
                ticks,
            } = block_data.block_type
            else {
                continue;
            };

            if !pressed {
                continue;
            }

            let new_ticks = ticks.saturating_sub(1);
            info!("Button tick, old: {ticks}, new: {new_ticks}");

            let mut new_pressed = pressed;

            if new_ticks == 0 {
                new_pressed = false;
            }

            queue.push(BlockChange::Update(UpdateRequest {
                position: pos.0,
                block_type: BlockType::StoneButton {
                    pressed: new_pressed,
                    attached_face,
                    ticks: new_ticks,
                },
            }))
        }
    }
}
