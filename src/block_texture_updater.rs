use crate::{
    BlockType,
    grid_plugin::{BlockChange, BlockChangeQueue, Grid, UpdateRequest},
    render::DirtyBlocks,
};
use bevy::prelude::*;

pub fn grass_to_dirt_updater(
    grid: Res<Grid>,
    dirty_blocks: Res<DirtyBlocks>,
    mut queue: ResMut<BlockChangeQueue>,
) {
    for position in &dirty_blocks.positions {
        if let Some(current_block_data) = grid.get(*position)
            && current_block_data.block_type == BlockType::StandardGrass
            && let Some(above_block_data) = grid.get(position + IVec3::Y)
            && above_block_data.block_type == BlockType::StandardGrass
        {
            queue.push(BlockChange::Update(UpdateRequest {
                position: *position,
                block_type: BlockType::Dirt,
            }));
        }
    }
}
