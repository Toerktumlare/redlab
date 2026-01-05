use bevy::prelude::*;

use crate::{
    BlockType,
    grid_plugin::{BlockChange, BlockChangeQueue, Grid, UpdateRequest},
    render::{
        DirtyRedstone,
        redstone_renderer::{Position, RedstoneLamp},
    },
};

const DIRS: [IVec3; 4] = [IVec3::X, IVec3::NEG_X, IVec3::Z, IVec3::NEG_Z];

pub fn propagate_redstone(
    grid: Res<Grid>,
    dirty_redstone: Res<DirtyRedstone>,
    mut queue: ResMut<BlockChangeQueue>,
) {
    for &pos in dirty_redstone.positions.iter() {
        let Some(block_data) = grid.get(pos) else {
            continue;
        };

        let BlockType::Dust { power: old, shape } = block_data.block_type else {
            continue;
        };

        let mut new_power = 0;

        for dir in DIRS {
            let neighbor_pos = pos + dir;
            if let Some(neighbor) = grid.get(neighbor_pos)
                && neighbor.block_type.is_conductor()
            {
                new_power = new_power.max(neighbor.block_type.power().saturating_sub(1));
            };
        }

        if new_power != old {
            queue.push(BlockChange::Update(UpdateRequest {
                position: pos,
                block_type: BlockType::Dust {
                    shape,
                    power: new_power,
                },
            }));
        }
    }
}

pub fn update_redstone_lamps(
    query: Query<&Position, With<RedstoneLamp>>,
    grid: Res<Grid>,
    mut queue: ResMut<BlockChangeQueue>,
) {
    for pos in &query {
        let pos = pos.0;
        let powered = if let Some(block_data) = grid.get(pos)
            && let BlockType::RedStoneLamp { powered } = block_data.block_type
        {
            powered
        } else {
            continue;
        };

        let mut has_power = false;

        for dir in DIRS {
            let neighbor_pos = pos + dir;
            if let Some(BlockType::Dust { power, .. }) =
                grid.get(neighbor_pos).map(|b| &b.block_type)
                && *power > 0
            {
                has_power = true;
                break;
            }
        }

        if has_power != powered {
            queue.push(BlockChange::Update(UpdateRequest {
                position: pos,
                block_type: BlockType::RedStoneLamp { powered: has_power },
            }));
        }
    }
}
