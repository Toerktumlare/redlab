use crate::{
    BlockType,
    grid_plugin::{BlockChange, BlockChangeQueue, Grid, UpdateRequest},
    render::DirtyBlocks,
};
use bevy::prelude::*;

pub fn update_grass_to_dirt(
    grid: Res<Grid>,
    dirty_blocks: Res<DirtyBlocks>,
    mut queue: ResMut<BlockChangeQueue>,
) {
    for &position in &dirty_blocks.positions {
        let Some(current) = grid.get(position) else {
            continue;
        };

        let BlockType::StandardGrass { .. } = current.block_type else {
            continue;
        };

        let above_pos = position + IVec3::Y;
        let Some(above) = grid.get(above_pos) else {
            continue;
        };

        if !matches!(above.block_type, BlockType::StandardGrass { .. }) {
            continue;
        }

        let new_block_type = BlockType::Dirt {
            power: current.block_type.power(),
        };

        if current.block_type == new_block_type {
            info!(
                pos = ?position,
                block = ?current.block_type,
                "skip grass->dirt (no state change)"
            );
            continue;
        }

        info!(
            pos = ?position,
            old = ?current.block_type,
            new = ?new_block_type,
            "enqueue grass->dirt"
        );

        queue.push(BlockChange::Update(UpdateRequest {
            position,
            block_type: new_block_type,
        }));
    }
}
