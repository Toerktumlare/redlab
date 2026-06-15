use std::{env::current_exe, thread::current};

use crate::scheduler::WorldEvent;

#[derive(Debug, Clone)]
pub struct HistoricTick {
    pub offset: u64,
    pub events: Vec<WorldEvent>,
}

impl HistoricTick {
    pub fn clear(&mut self) {
        self.events.clear();
        self.offset = 0;
    }

    pub fn add_event(&mut self, event: WorldEvent) {
        self.events.push(event);
    }
}

#[derive(Debug)]
pub enum RecordState {
    Running,
    Stopping,
    Stopped,
    Scrubbing,
}

#[derive(Debug)]
pub struct TimeMachine {
    pub history: Vec<HistoricTick>,
    pub starting_tick: u64,
    write_idx: usize,
    size: usize,
    scrub_idx: usize,
    state: RecordState,
}

impl TimeMachine {
    pub fn new(sec: u32) -> Self {
        let capacity = sec * 20;
        let mut history = Vec::with_capacity(capacity as usize);
        for _ in 0..capacity {
            let ht = HistoricTick {
                events: Vec::with_capacity(1000),
                offset: 0,
            };
            history.push(ht);
        }

        Self {
            history,
            starting_tick: 0,
            write_idx: 0,
            size: 0,
            scrub_idx: 0,
            state: RecordState::Stopped,
        }
    }

    pub(crate) fn record(&mut self, current_tick: u64) {
        if matches!(self.state, RecordState::Running) {
            let current = &mut self.history[self.write_idx];
            current.clear();
            current.offset = current_tick.saturating_sub(self.starting_tick);
        }
    }

    pub(crate) fn add_event(&mut self, event: WorldEvent) {
        if matches!(self.state, RecordState::Running | RecordState::Stopping) {
            self.history[self.write_idx].add_event(event);
        }
    }

    pub(crate) fn scrub_back(&mut self) -> Option<&HistoricTick> {
        if matches!(self.state, RecordState::Scrubbing) {
            let len = self.history.len();
            self.scrub_idx = (self.scrub_idx + len - 1) % len;
            return Some(&self.history[self.scrub_idx]);
        }
        None
    }

    // pub(crate) fn scrub_forward(&mut self) -> Option<&HistoricTick> {
    //     if !self.is_scrubbing || self.scrub_idx == self.write_idx {
    //         return None;
    //     }

    //     self.scrub_idx = (self.scrub_idx + 1) % self.history.len();

    //     Some(&self.history[self.scrub_idx])
    // }

    pub(crate) fn end_tick(&mut self) {
        if matches!(self.state, RecordState::Running | RecordState::Stopping) {
            self.write_idx = (self.write_idx + 1) % self.history.len();
        }

        if matches!(self.state, RecordState::Stopping) {
            self.state = RecordState::Stopped;
            self.scrub_idx = self.write_idx;
        }
        // think about this
        self.size += 1;
    }

    pub(crate) fn start(&mut self, current_tick: u64) {
        self.size = 0;
        self.state = RecordState::Running;
        self.starting_tick = current_tick;
    }

    pub(crate) fn stop(&mut self) {
        self.state = RecordState::Stopping
    }
}

impl Iterator for TimeMachine {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}
