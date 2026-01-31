use bevy::prelude::*;
use std::collections::HashMap;

use crate::{
    BlockData, BlockType,
    blocks::{NeighbourUpdate, RecomputedResult, Tickable},
    redstone::{GlobalTick, NotifyDelay, Scheduler, Tick},
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
    neighbor_tick: Vec<NeighbourUpdate>,
}

impl Place {
    pub(crate) fn new(
        block_type: BlockType,
        position: IVec3,
        visual_change: bool,
        self_tick: Option<NotifyDelay>,
        neighbor_tick: Vec<NeighbourUpdate>,
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
    neighbor_tick: Vec<NeighbourUpdate>,
}

impl Remove {
    pub(crate) fn new(
        position: IVec3,
        visual_change: bool,
        self_tick: Option<NotifyDelay>,
        neighbor_tick: Vec<NeighbourUpdate>,
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

pub fn grid_apply_changes(
    mut queue: ResMut<BlockChangeQueue>,
    mut grid: ResMut<Grid>,
    mut dirty_blocks: ResMut<DirtyBlocks>,
    mut dirty_render: ResMut<DirtyRender>,
    mut scheduler: ResMut<Scheduler>,
    global_tick: Res<GlobalTick>,
) {
    let now = global_tick.read();
    for change in queue.drain() {
        if let Some(position) = apply_change(&mut grid, &change) {
            info!("Current block proccessed: {}", position);
            schedule_self_tick(position, &mut scheduler, now, &change);

            schedule_ticks_and_mark_neighbours(
                position,
                &grid,
                &mut scheduler,
                &mut dirty_blocks,
                &change,
                now,
            );

            mark_for_redraw(position, &mut dirty_render, &change);
        }
    }

    while let Some(position) = scheduler.immediate.pop_front() {
        let block_type = match grid.get_blocktype(position) {
            Some(bt) => *bt,
            None => continue,
        };

        let result = block_type.on_tick(&grid, position);

        if let RecomputedResult::Changed {
            new_block,
            visual_update,
            self_tick,
            neighbor_tick,
        } = result
        {
            match new_block {
                Some(block_type) => grid.insert(position, BlockData { block_type }),
                None => grid.remove(position),
            }

            if let Some(self_tick) = self_tick {
                scheduler.schedule(position, &self_tick, now);
            }

            for update in neighbor_tick {
                let position = position + update.position;
                if grid.get(position).is_some() {
                    info!("Scheduling neighbour: {}", position);
                    scheduler.schedule(position, &update.notification, now);

                    info!("Marking block as dirty: {}", position);
                    dirty_blocks.mark(position);
                }
            }

            if visual_update {
                dirty_render.mark(position);
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

fn apply_change(grid: &mut Grid, block_change: &BlockChange) -> Option<IVec3> {
    match block_change {
        BlockChange::Place(event) => try_place(grid, event),
        BlockChange::Remove(event) => try_remove(grid, event.position),
    }
}

fn schedule_self_tick(position: IVec3, scheduler: &mut Scheduler, now: Tick, change: &BlockChange) {
    if let Some(self_tick) = match change {
        BlockChange::Place(event) => &event.self_tick,
        BlockChange::Remove(event) => &event.self_tick,
    } {
        info!("Scheduling self: {}", position);
        scheduler.schedule(position, self_tick, now);
    }
}

fn schedule_ticks_and_mark_neighbours(
    position: IVec3,
    grid: &Grid,
    scheduler: &mut Scheduler,
    dirty_blocks: &mut DirtyBlocks,
    change: &BlockChange,
    now: Tick,
) {
    let neighbor_tick = match change {
        BlockChange::Place(event) => &event.neighbor_tick,
        BlockChange::Remove(event) => &event.neighbor_tick,
    };

    for n_update in neighbor_tick {
        let position = position + n_update.position;
        if grid.get(position).is_some() {
            info!("Scheduling neighbour: {}", position);
            scheduler.schedule(position, &n_update.notification, now);

            info!("Marking block as dirty: {}", position);
            dirty_blocks.mark(position);
        }
    }
}

fn mark_for_redraw(position: IVec3, dirty_render: &mut DirtyRender, change: &BlockChange) {
    let visual_change = match change {
        BlockChange::Place(event) => event.visual_change,
        BlockChange::Remove(event) => event.visual_change,
    };

    if visual_change {
        dirty_render.mark(position);
    }
}
