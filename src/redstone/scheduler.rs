use bevy::prelude::*;
use std::collections::VecDeque;

use bevy::{
    math::IVec3,
    platform::collections::{HashMap, HashSet},
};

pub type Tick = u64;

#[derive(Debug, Clone, PartialEq)]
pub enum NotifyDelay {
    Immediate,
    NextTick,
    In(Tick),
}

#[derive(Resource, Default)]
pub struct Scheduler {
    pub immediate: VecDeque<IVec3>,
    scheduled: HashMap<u64, HashSet<IVec3>>,
}

impl Scheduler {
    pub fn schedule(&mut self, pos: IVec3, delay: &NotifyDelay, now: Tick) {
        match delay {
            NotifyDelay::Immediate => {
                self.immediate.push_back(pos);
            }
            NotifyDelay::NextTick => {
                self.schedule_at(pos, now + 1);
            }
            NotifyDelay::In(ticks) => {
                self.schedule_at(pos, now + ticks);
            }
        }
    }

    pub fn advance(&mut self, now: Tick) {
        let positions = self.scheduled.remove(&now).unwrap_or_default();
        self.immediate.extend(positions);
    }

    fn schedule_at(&mut self, pos: IVec3, tick: u64) {
        self.scheduled.entry(tick).or_default().insert(pos);
    }

    pub fn immediate_queue(&self) -> impl Iterator<Item = (&u64, &HashSet<IVec3>)> {
        let mut queue: Vec<_> = self.scheduled.iter().collect();
        queue.sort_by_key(|(tick, _)| *tick);
        queue.into_iter()
    }
}
