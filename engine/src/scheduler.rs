use std::collections::BTreeMap;

use crate::grid::{BlockPos, BlockState};

#[derive(Debug, Clone, Copy)]
pub enum WorldEvent {
    Block(BlockEvent),
    Schedule(ScheduleEvent),
}

#[derive(Debug, Clone, Copy)]
pub enum BlockEvent {
    Set(BlockPos, BlockState),
    Update(BlockPos, BlockState),
    Remove(BlockPos),
}

#[derive(Debug, Clone, Copy)]
pub enum ScheduleEvent {
    Tick(BlockPos),
}

#[derive(Debug)]
pub struct Scheduler {
    pub immediate: Vec<WorldEvent>,
    pub delayed: BTreeMap<u64, Vec<WorldEvent>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            immediate: Vec::with_capacity(1024),
            delayed: BTreeMap::new(),
        }
    }

    pub fn block_update(&mut self, pos: BlockPos, state: BlockState) {
        self.immediate
            .push(WorldEvent::Block(BlockEvent::Update(pos, state)));
    }

    pub fn block_set(&mut self, pos: BlockPos, state: BlockState) {
        self.immediate
            .push(WorldEvent::Block(BlockEvent::Set(pos, state)));
    }

    pub fn block_remove(&mut self, pos: BlockPos) {
        self.immediate
            .push(WorldEvent::Block(BlockEvent::Remove(pos)));
    }

    pub fn schedule_tick(&mut self, pos: BlockPos) {
        self.immediate
            .push(WorldEvent::Schedule(ScheduleEvent::Tick(pos)));
    }

    pub fn schedule_future(&mut self, pos: BlockPos, in_ticks: u64) {
        self.delayed
            .entry(in_ticks)
            .or_default()
            .push(WorldEvent::Schedule(ScheduleEvent::Tick(pos)));
    }

    pub fn advance(&mut self, current_tick: u64) {
        if let Some(event) = self.delayed.remove(&current_tick) {
            self.immediate.extend(event);
        }
    }

    pub fn flush_events<F>(&mut self, mut handler: F)
    where
        F: FnMut(WorldEvent),
    {
        let mut ready = std::mem::take(&mut self.immediate);
        for event in ready.drain(..) {
            handler(event);
        }
        self.immediate = ready;
    }
}
