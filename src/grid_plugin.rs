use bevy::prelude::*;
use std::collections::HashMap;

use crate::{
    BlockData, BlockType,
    redstone::{GlobalTick, NotifyDelay, Scheduler},
    render::{DirtyBlocks, DirtyRender},
};

#[derive(Event, Clone, Debug)]
pub enum BlockChange {
    Place(Place),
    Remove(Remove),
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

    pub fn get_blocktype(&self, pos: IVec3) -> Option<&BlockType> {
        self.blocks.get(&pos).map(|b| &b.block_type)
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
pub struct Place {
    position: IVec3,
    block_type: BlockType,
    visual_change: bool,
    self_tick: Option<NotifyDelay>,
    neighbor_tick: Option<NotifyDelay>,
}

impl Place {
    pub(crate) fn new(
        block_type: BlockType,
        position: IVec3,
        visual_change: bool,
        self_tick: Option<NotifyDelay>,
        neighbor_tick: Option<NotifyDelay>,
    ) -> Self {
        Self {
            position,
            block_type,
            visual_change,
            self_tick,
            neighbor_tick,
        }
    }
}

#[derive(Event, Clone, Debug)]
pub struct Remove {
    position: IVec3,
    visual_change: bool,
    self_tick: Option<NotifyDelay>,
    neighbor_tick: Option<NotifyDelay>,
}

impl Remove {
    pub(crate) fn new(
        position: IVec3,
        visual_change: bool,
        self_tick: Option<NotifyDelay>,
        neighbor_tick: Option<NotifyDelay>,
    ) -> Self {
        Self {
            position,
            visual_change,
            self_tick,
            neighbor_tick,
        }
    }
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

const ALL_DIRS: [IVec3; 6] = [
    IVec3::NEG_Y,
    IVec3::Y,
    IVec3::NEG_Z,
    IVec3::Z,
    IVec3::NEG_X,
    IVec3::X,
];

pub fn grid_apply_changes(
    mut queue: ResMut<BlockChangeQueue>,
    mut grid: ResMut<Grid>,
    mut dirty_blocks: ResMut<DirtyBlocks>,
    mut dirty_render: ResMut<DirtyRender>,
    mut scheduler: ResMut<Scheduler>,
    global_tick: Res<GlobalTick>,
) {
    for change in queue.drain() {
        let (inserted_position, visual_change, self_tick, neighbor_tick) = match change {
            BlockChange::Place(event) => (
                try_place(&mut grid, &event),
                event.visual_change,
                event.self_tick,
                event.neighbor_tick,
            ),
            BlockChange::Remove(event) => (
                try_remove(&mut grid, event.position),
                event.visual_change,
                event.self_tick,
                event.neighbor_tick,
            ),
        };

        if let Some(inserted_position) = inserted_position {
            let now = global_tick.read();

            if let Some(self_tick) = self_tick {
                info!("Scheduling self: {}", inserted_position);
                scheduler.schedule(inserted_position, &self_tick, now);
            }

            for dir in ALL_DIRS {
                let position = inserted_position + dir;
                if grid.get(position).is_some() {
                    if let Some(neighbor_tick) = &neighbor_tick {
                        info!("Scheduling neighbour: {}", position);
                        scheduler.schedule(inserted_position, neighbor_tick, now);
                    }

                    dirty_blocks.mark(position);
                }
            }

            if visual_change {
                info!("marked: {} to render", inserted_position);
                dirty_render.mark(inserted_position);
            }
        }
    }
}

fn try_place(grid: &mut Grid, event: &Place) -> Option<IVec3> {
    let position = event.position;
    let block_type = event.block_type;

    grid.insert(position, BlockData { block_type });
    Some(position)
}

fn try_remove(grid: &mut Grid, position: IVec3) -> Option<IVec3> {
    if grid.get_mut(position).is_some() {
        grid.remove(position);
        return Some(position);
    }
    None
}
