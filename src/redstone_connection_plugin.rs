use bevy::prelude::*;

use crate::{
    BlockData, BlockType,
    grid_plugin::{BlockChange, BlockChangeQueue, Grid},
    render::DirtyRedstone,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JunctionType {
    Dot,
    Vertical,
    Horizontal,
    CornerNE,
    CornerNW,
    CornerSE,
    CornerSW,
    TNorth,
    TSouth,
    TEast,
    TWest,
    Cross,
}

fn get_junction_type(connections: &[bool]) -> JunctionType {
    info!("{connections:?}");
    // N, S, E, W
    match connections {
        [false, false, false, false] => JunctionType::Dot,
        [true, true, true, true] => JunctionType::Cross,
        [true, false, true, true] => JunctionType::TNorth,
        [true, true, true, false] => JunctionType::TEast,
        [true, true, false, true] => JunctionType::TWest,
        [false, true, true, true] => JunctionType::TSouth,
        [true, true, false, false] => JunctionType::Vertical,
        [true, false, false, false] => JunctionType::Vertical,
        [false, true, false, false] => JunctionType::Vertical,
        [false, false, true, true] => JunctionType::Horizontal,
        [false, false, false, true] => JunctionType::Horizontal,
        [false, false, true, false] => JunctionType::Horizontal,
        [false, true, true, false] => JunctionType::CornerNE,
        [true, false, false, true] => JunctionType::CornerSW,
        [true, false, true, false] => JunctionType::CornerSE,
        [false, true, false, true] => JunctionType::CornerNW,
        _ => JunctionType::Dot,
    }
}

pub fn update_redstone_system(
    dirty_blocks: Res<DirtyRedstone>,
    grid: Res<Grid>,
    mut queue: ResMut<BlockChangeQueue>,
) {
    for position in dirty_blocks.positions.iter() {
        let position = *position;
        let Some(block_data) = grid.get(position) else {
            continue;
        };

        match block_data.block_type {
            BlockType::Dust { power, .. } => {
                let junction = resolve_junction(position, &grid);
                queue.push(BlockChange::Update(crate::grid_plugin::UpdateRequest {
                    position,
                    block_type: BlockType::Dust {
                        shape: junction,
                        power,
                    },
                }));
            }
            _ => continue,
        };
    }
}

fn resolve_junction(position: IVec3, grid: &Grid) -> JunctionType {
    let dir = [
        &(position - IVec3::Z), // north
        &(position + IVec3::Z), // south
        &(position + IVec3::X), // east
        &(position - IVec3::X), // west
    ];

    let mut connections = [false; 4];

    for (i, position) in dir.iter().enumerate() {
        let Some(block_data) = grid.get(**position) else {
            continue;
        };
        connections[i] = has_redstone(block_data);
    }

    get_junction_type(&connections)
}

fn has_redstone(block_data: &BlockData) -> bool {
    matches!(
        block_data.block_type,
        BlockType::Dust { .. }
            | BlockType::RedStone
            | BlockType::RedStoneLamp { .. }
            | BlockType::RedStoneTorch { .. }
    )
}
