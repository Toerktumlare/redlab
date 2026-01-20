use bevy::prelude::*;

use crate::{
    blocks::{Block, RecomputedResult},
    grid_plugin::{BlockChange, BlockChangeQueue, Grid, Place, Remove},
    render::{DirtyBlocks, DirtyRender},
};

pub fn recalculate_dirty_blocks(
    mut dirty_blocks: ResMut<DirtyBlocks>,
    grid: Res<Grid>,
    mut render_dirty: ResMut<DirtyRender>,
    mut queue: ResMut<BlockChangeQueue>,
) {
    for position in dirty_blocks.drain() {
        let Some(block_data) = grid.get(position) else {
            continue;
        };

        let result = block_data.block_type.neighbor_changed(&grid, position);

        info!(?position, ?result);

        match result {
            RecomputedResult::Changed {
                new_block,
                visual_update,
                self_tick,
                neighbor_tick,
            } => {
                match new_block {
                    Some(new_block) => queue.push(BlockChange::Place(Place::new(
                        new_block,
                        position,
                        visual_update,
                        self_tick,
                        neighbor_tick,
                    ))),
                    None => queue.push(BlockChange::Remove(Remove::new(
                        position,
                        visual_update,
                        self_tick,
                        neighbor_tick,
                    ))),
                }

                if visual_update {
                    render_dirty.mark(position);
                }
            }
            RecomputedResult::Unchanged => {}
        }
    }
}
