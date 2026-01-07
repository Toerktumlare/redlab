use bevy::prelude::*;
use std::collections::HashMap;

use crate::{BlockData, BlockType, render::DirtyBlocks, render::DirtyRedstone};

#[derive(Event, Clone, Debug)]
pub enum BlockChange {
    Place(PlaceRequest),
    Update(UpdateRequest),
    Remove(RemoveRequest),
}

#[derive(Resource, Default)]
pub struct BlockChangeQueue {
    changes: Vec<BlockChange>,
}

impl BlockChangeQueue {
    pub fn push(&mut self, change: BlockChange) {
        self.changes.push(change);
    }
    pub fn drain(&mut self) -> vec::Drain<'_, BlockChange> {
        self.changes.drain(..)
    }
}

pub struct GridPlugin;

#[derive(Resource, Default)]
pub struct Grid {
    blocks: HashMap<IVec3, BlockData>,
}

impl Grid {
    pub fn get(&self, pos: IVec3) -> Option<&BlockData> {
        self.blocks.get(&pos)
    }

    pub fn get_mut(&mut self, pos: IVec3) -> Option<&mut BlockData> {
        self.blocks.get_mut(&pos)
    }

    pub fn insert(&mut self, pos: IVec3, data: BlockData) {
        self.blocks.insert(pos, data);
    }

    pub fn remove(&mut self, pos: IVec3) {
        self.blocks.remove(&pos);
    }
}

#[derive(Event, Clone, Debug)]
pub struct PlaceRequest {
    pub position: IVec3,
    pub normal: IVec3,
    pub block_type: BlockType,
}

#[derive(Event, Default, Clone, Debug)]
pub struct RemoveRequest {
    pub position: IVec3,
}

#[derive(Event, Clone, Debug)]
pub struct UpdateRequest {
    pub position: IVec3,
    pub block_type: BlockType,
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Grid>()
            .init_resource::<BlockChangeQueue>();
    }
}

pub fn queue_block_change(event: On<BlockChange>, mut queue: ResMut<BlockChangeQueue>) {
    info!("Queueing up event: {:?}", event.event());
    queue.push(event.event().clone());
}

pub fn grid_apply_changes(
    mut queue: ResMut<BlockChangeQueue>,
    mut grid: ResMut<Grid>,
    mut dirty_blocks: ResMut<DirtyBlocks>,
    mut dirty_redstone: ResMut<DirtyRedstone>,
) {
    for change in queue.drain() {
        let changed_positions = match change {
            BlockChange::Place(event) => try_place(&mut grid, &event),
            BlockChange::Update(event) => try_update(&mut grid, &event),
            BlockChange::Remove(event) => try_remove(&mut grid, event.position),
        };

        if let Some(changed_positions) = changed_positions {
            for pos in changed_positions {
                if let Some(block_data) = grid.get(pos) {
                    match block_data.block_type {
                        BlockType::RedStoneLamp { .. }
                        | BlockType::Dust { .. }
                        | BlockType::RedStoneTorch { .. }
                        | BlockType::StoneButton { .. } => dirty_redstone.mark(pos),
                        _ => dirty_blocks.mark(pos),
                    }
                } else {
                    // Ugly solution, main renderer will delete any block_type
                    // Should be its own system
                    dirty_blocks.mark(pos);
                };
            }
        }
    }
}

fn try_place(grid: &mut Grid, event: &PlaceRequest) -> Option<Vec<IVec3>> {
    let position = event.position + event.normal;
    let block_type = event.block_type;

    grid.insert(position, BlockData { block_type });

    Some(neighbor_positions(position))
}

fn try_update(grid: &mut Grid, event: &UpdateRequest) -> Option<Vec<IVec3>> {
    let position = event.position;
    let new_block_type = event.block_type;

    let block_data = grid.get_mut(position)?;

    if block_data.block_type != new_block_type {
        block_data.block_type = new_block_type;
        Some(neighbor_positions(position))
    } else {
        None
    }
}

fn try_remove(grid: &mut Grid, position: IVec3) -> Option<Vec<IVec3>> {
    if grid.get_mut(position).is_some() {
        grid.remove(position);
        return Some(neighbor_positions(position));
    }
    None
}

fn neighbor_positions(position: IVec3) -> Vec<IVec3> {
    const DIRS: [IVec3; 6] = [
        IVec3::new(1, 0, 0),
        IVec3::new(-1, 0, 0),
        IVec3::new(0, 1, 0),
        IVec3::new(0, -1, 0),
        IVec3::new(0, 0, 1),
        IVec3::new(0, 0, -1),
    ];

    let mut out = Vec::with_capacity(7);
    out.push(position);
    for d in DIRS {
        out.push(position + d);
    }
    out
}
