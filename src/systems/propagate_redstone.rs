use bevy::prelude::*;

use crate::{
    BlockData, BlockType,
    grid_plugin::{BlockChange, BlockChangeQueue, Grid, UpdateRequest},
    render::{
        DirtyBlocks, DirtyRedstone,
        redstone_renderer::{Position, RedstoneLamp},
    },
};

const ALL_DIRS: [IVec3; 6] = [
    IVec3::X,
    IVec3::NEG_X,
    IVec3::Z,
    IVec3::NEG_Z,
    IVec3::Y,
    IVec3::NEG_Y,
];

const DIRS: [IVec3; 4] = [IVec3::X, IVec3::NEG_X, IVec3::Z, IVec3::NEG_Z];

pub fn propagate_strong_power(
    grid: Res<Grid>,
    dirty_redstone: Res<DirtyRedstone>,
    dirty_blocks: Res<DirtyBlocks>,
    mut queue: ResMut<BlockChangeQueue>,
) {
    let dirty_redstone = dirty_redstone.positions.iter();
    let dirty_blocks = dirty_blocks.positions.iter();
    let dirty = dirty_redstone.chain(dirty_blocks);

    for &position in dirty {
        let Some(block_data) = grid.get(position) else {
            continue;
        };

        let block_type = block_data.block_type;

        let mut new_strong = 0;

        if block_type.is_insulator() {
            continue;
        }

        for dir in ALL_DIRS {
            let neighbour_pos = position + dir;
            let Some(neighbour_data) = grid.get(neighbour_pos) else {
                continue;
            };

            let neighbour_block = neighbour_data.block_type;
            new_strong = new_strong.max(neighbour_block.strong_power_emitted_to(
                position,
                neighbour_pos,
                &block_type,
            ));
        }

        if block_type.strong_power() != new_strong {
            queue.push(BlockChange::Update(UpdateRequest {
                position,
                block_type: block_type.with_weak_power(new_strong),
            }));
        }
    }
}

pub fn propagate_torch_power(
    grid: Res<Grid>,
    dirty_redstone: Res<DirtyRedstone>,
    dirty_blocks: Res<DirtyBlocks>,
    mut queue: ResMut<BlockChangeQueue>,
) {
    let dirty_redstone = dirty_redstone.positions.iter();
    let dirty_blocks = dirty_blocks.positions.iter();
    let dirty = dirty_redstone.chain(dirty_blocks);

    for &position in dirty {
        let Some(block_data) = grid.get(position) else {
            continue;
        };

        let block_type = block_data.block_type;

        if let BlockType::RedStoneTorch { on, attached_face } = block_type {
            let neighbour_pos = position + attached_face;
            if let Some(block_data) = grid.get(neighbour_pos) {
                let attached_block_has_power = block_data.block_type.is_powered();
                let new_on = !attached_block_has_power;
                if on != new_on {
                    queue.push(BlockChange::Update(UpdateRequest {
                        position,
                        block_type: BlockType::RedStoneTorch {
                            on: new_on,
                            attached_face,
                        },
                    }));
                }
            }
        }
    }
}

pub fn propagate_dust_power(
    grid: Res<Grid>,
    dirty_redstone: Res<DirtyRedstone>,
    dirty_blocks: Res<DirtyBlocks>,
    mut queue: ResMut<BlockChangeQueue>,
) {
    let dirty_redstone = dirty_redstone.positions.iter();
    let dirty_blocks = dirty_blocks.positions.iter();
    let dirty = dirty_redstone.chain(dirty_blocks);

    for &position in dirty {
        let Some(block_data) = grid.get(position) else {
            continue;
        };

        let current_block_type = block_data.block_type;

        if !matches!(current_block_type, BlockType::Dust { .. }) {
            continue;
        }

        let mut new_power = 0;
        for dir in ALL_DIRS {
            let neighbour_pos = position + dir;
            let Some(neighbour_data) = grid.get(neighbour_pos) else {
                continue;
            };

            let neighbour_block = neighbour_data.block_type;

            new_power = new_power.max(neighbour_block.strong_power_emitted_to(
                position,
                neighbour_pos,
                &neighbour_block,
            ));
            new_power = new_power.max(
                neighbour_block
                    .weak_power_emitted(position, neighbour_pos, &current_block_type)
                    .saturating_sub(1),
            );

            // vertical corners: above and below
            for &y_offset in &[IVec3::Y, IVec3::NEG_Y] {
                for dir in DIRS {
                    let neighbour_pos = position + dir + y_offset;
                    if let Some(neighbour_block) = grid.get_blocktype(neighbour_pos) {
                        new_power = new_power.max(
                            neighbour_block
                                .weak_power_emitted(position, neighbour_pos, &current_block_type)
                                .saturating_sub(1),
                        );
                    }
                }
            }
        }

        info!(
            "Current block: {:?}, new_power: {}",
            current_block_type, new_power
        );
        if current_block_type.weak_power() != new_power {
            queue.push(BlockChange::Update(UpdateRequest {
                position,
                block_type: current_block_type.with_weak_power(new_power),
            }));
        }
    }
}

pub fn propagate_block_power(
    grid: Res<Grid>,
    dirty_redstone: Res<DirtyRedstone>,
    dirty_blocks: Res<DirtyBlocks>,
    mut queue: ResMut<BlockChangeQueue>,
) {
    let dirty_redstone = dirty_redstone.positions.iter();
    let dirty_blocks = dirty_blocks.positions.iter();
    let dirty = dirty_redstone.chain(dirty_blocks);

    for &position in dirty {
        let Some(block_data) = grid.get(position) else {
            continue;
        };

        let block_type = block_data.block_type;

        if !matches!(block_type, BlockType::StandardGrass { .. }) {
            continue;
        }

        let mut new_power = 0;
        for dir in ALL_DIRS {
            let neighbour_pos = position + dir;
            let Some(neighbour_data) = grid.get(neighbour_pos) else {
                continue;
            };

            let neighbour_block = neighbour_data.block_type;

            if !neighbour_block.is_emitter() {
                continue;
            }

            new_power = resolve_full_power(&block_type, position, neighbour_pos);
        }

        info!("Current block: {:?}, new_power: {}", block_type, new_power);
        if block_type.weak_power() != new_power {
            queue.push(BlockChange::Update(UpdateRequest {
                position,
                block_type: block_type.with_weak_power(new_power),
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

        for dir in ALL_DIRS {
            let neighbor_pos = pos + dir;
            if let Some(BlockType::Dust { power, .. }) =
                grid.get(neighbor_pos).map(|b| &b.block_type)
                && power.weak > 0
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

pub fn resolve_full_power(
    block_type: &BlockType,
    current_block: IVec3,
    neighbour_block: IVec3,
) -> u8 {
    let mut new_power = 0;
    new_power = new_power.max(block_type.strong_power_emitted_to(
        current_block,
        neighbour_block,
        block_type,
    ));
    new_power =
        new_power.max(block_type.weak_power_emitted(current_block, neighbour_block, block_type));
    new_power
}
