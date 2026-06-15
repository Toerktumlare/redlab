use crate::{
    grid::{BlockPos, BlockState, Grid},
    scheduler::Scheduler,
};

use std::fmt::Debug;

pub trait Block: Debug + Base + Tickable {}

pub trait Base {
    fn on_placement(
        &self,
        grid: &Grid,
        position: BlockPos,
        state: BlockState,
        schedule: &mut Scheduler,
    );

    fn neighbor_changed(
        &self,
        grid: &Grid,
        pos: BlockPos,
        state: BlockState,
        schedule: &mut Scheduler,
    );

    fn try_place(
        &self,
        grid: &Grid,
        pos: BlockPos,
        state: BlockState,
        scheduler: &mut Scheduler,
    ) -> bool;

    fn on_remove(&self, grid: &Grid, pos: BlockPos, state: BlockState, scheduler: &mut Scheduler) {}

    fn is_tickable(&self) -> bool {
        false
    }
}

pub trait Tickable {
    fn on_tick(&self, grid: &Grid, pos: BlockPos, state: BlockState, scheduler: &mut Scheduler);
    fn power(
        &self,
        grid: &Grid,
        pos: BlockPos,
        state: BlockState,
        askers_pos: BlockPos,
        scheduler: &mut Scheduler,
    ) -> u8 {
        0
    }
}

#[derive(Debug)]
pub struct AirBlock;

impl Base for AirBlock {
    fn on_placement(
        &self,
        grid: &Grid,
        pos: BlockPos,
        state: BlockState,
        scheduler: &mut Scheduler,
    ) {
    }

    fn neighbor_changed(
        &self,
        grid: &Grid,
        pos: BlockPos,
        state: BlockState,
        scheduler: &mut Scheduler,
    ) {
    }

    fn try_place(
        &self,
        grid: &Grid,
        pos: BlockPos,
        state: BlockState,
        scheduler: &mut Scheduler,
    ) -> bool {
        true
    }

    fn is_tickable(&self) -> bool {
        false
    }
}

impl Tickable for AirBlock {
    fn on_tick(&self, grid: &Grid, pos: BlockPos, state: BlockState, scheduler: &mut Scheduler) {}
}
