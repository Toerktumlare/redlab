use crate::{
    blocks::{Base, Block, Tickable},
    grid::*,
    scheduler::Scheduler,
    world::{Registry, World},
};
use pretty_assertions::assert_eq;

#[derive(Debug)]
pub struct TestBlock;

impl Block for TestBlock {}

impl Base for TestBlock {
    fn on_placement(
        &self,
        _grid: &Grid,
        _position: BlockPos,
        _state: BlockState,
        _scheduler: &mut Scheduler,
    ) {
    }

    fn neighbor_changed(
        &self,
        _grid: &Grid,
        _pos: BlockPos,
        _state: BlockState,
        _schedule: &mut Scheduler,
    ) {
    }

    fn try_place(
        &self,
        _grid: &Grid,
        _pos: BlockPos,
        _state: BlockState,
        _scheduler: &mut Scheduler,
    ) -> bool {
        true
    }

    fn on_remove(
        &self,
        _grid: &Grid,
        _pos: BlockPos,
        _state: BlockState,
        _scheduler: &mut Scheduler,
    ) {
    }

    fn is_tickable(&self) -> bool {
        true
    }
}

impl Tickable for TestBlock {
    fn on_tick(&self, _grid: &Grid, pos: BlockPos, _state: BlockState, scheduler: &mut Scheduler) {
        let new_state = BlockState::new(2, 0); // ID 2
        scheduler.block_update(pos, new_state);
    }

    fn power(
        &self,
        _grid: &Grid,
        _pos: BlockPos,
        _state: BlockState,
        _askers_pos: BlockPos,
        _schedule: &mut Scheduler,
    ) -> u8 {
        0
    }
}

fn setup() -> World {
    let mut reg = Registry::new(1024);
    reg.register(TestBlock);
    let grid = Grid::new(3, 3, 3);
    let sched = Scheduler::new();
    World::new(grid, reg, sched)
}

#[test]
fn test_block_state_bit_packing() {
    // Testing your 10-bit ID and 22-bit Payload
    let id = 1023; // Max 10-bit value
    let payload = 5000;
    let state = BlockState::new(id, payload);

    assert_eq!(state.id(), 1023);
    assert_eq!(state.payload(), 5000);

    // Ensure ID doesn't bleed into payload
    let state_over_limit = BlockState::new(2048, 0); // 2048 is 11th bit
    assert_eq!(state_over_limit.id(), 0); // Should be masked out
}

#[test]
fn test_immediate_cascade() {
    let mut world = setup();
    let pos = BlockPos::new(0, 0, 0);

    // Manually trigger a block set event
    world.scheduler.block_set(pos, BlockState::new(1, 99));

    world.update();

    // Verify the grid updated
    let state = world.grid.get_block(pos);
    assert_eq!(state.id(), 1);
    assert_eq!(state.payload(), 99);
}

#[test]
fn test_safety_counter_prevents_hang() {
    let mut world = setup();

    #[derive(Debug)]
    struct InfiniteBlock;
    impl Base for InfiniteBlock {
        fn is_tickable(&self) -> bool {
            true
        }
        fn on_placement(&self, _: &Grid, _: BlockPos, _: BlockState, _: &mut Scheduler) {}
        fn neighbor_changed(&self, _: &Grid, _: BlockPos, _: BlockState, _: &mut Scheduler) {}
        fn try_place(&self, _: &Grid, _: BlockPos, _: BlockState, _: &mut Scheduler) -> bool {
            true
        }
    }
    impl Tickable for InfiniteBlock {
        fn on_tick(&self, _: &Grid, pos: BlockPos, _: BlockState, sched: &mut Scheduler) {
            sched.schedule_tick(pos);
        }
    }
    impl Block for InfiniteBlock {}

    let id = world.registry.register(InfiniteBlock);
    let pos = BlockPos::new(1, 1, 1);

    world.grid.replace_block(pos, BlockState::new(id as u32, 0));
    world.scheduler.schedule_tick(pos);
    world.update();

    assert!(world.scheduler.immediate.is_empty());
}

#[test]
fn should_store_events_in_time_machine() {
    let mut world = setup();

    world.start_recording();

    let pos = BlockPos::new(0, 0, 0);
    let first_state = BlockState::new(0, 1);
    let second_state = BlockState::new(0, 2);
    world.scheduler.block_set(pos, first_state);
    world.scheduler.block_set(pos, second_state);
    world.update();

    let pos = BlockPos::new(0, 0, 0);
    let third_state = BlockState::new(0, 3);
    world.scheduler.block_set(pos, third_state);
    world.tick();
    world.update();

    assert_eq!(world.time_machine.history[0].events.len(), 2);
    assert_eq!(world.time_machine.history[0].offset, 0);
    assert_eq!(world.time_machine.history[1].events.len(), 1);
    assert_eq!(world.time_machine.history[1].offset, 1);
}

#[test]
fn should_write_events_in_a_ring() {
    let mut world = setup();
    world.start_recording();

    for i in 0..=20 {
        let pos = BlockPos::new(0, 0, 0);
        let state = BlockState::new(0, i);
        world.scheduler.block_set(pos, state);
        world.update();
    }

    // assert_eq!(world.time_machine.history[0].tick, 20);
    assert_eq!(world.time_machine.history[0].events.len(), 1);
}

#[test]
fn should_record_scheduled_jobs() {
    // todo, do we want to see what was scheduled during each tick?
    // or do we just want to look at all events linearily?
}

#[test]
fn test_scrub_backward_time_one_event() {
    let mut world = setup();
    world.start_recording();

    for i in 0..5 {
        let pos = BlockPos::new(0, 0, 0);
        let state = BlockState::new(0, i);
        world.scheduler.block_set(pos, state);
        world.update();
        world.tick();
    }

    world.stop_recording();

    world.reverse(1);

    // assert_eq!(world.time_machine.history[0].tick, 20);
    // assert_eq!(world.time_machine.history[0].events.len(), 1);
}

#[test]
fn test_scrub_forward_time() {
    // just apply tick backwards and forwards
}

#[test]
fn scrub_backward_time_n_ticks() {
    // move multiple ticks
}

#[test]
fn scrub_backward_time_n_events() {
    // move single events
}
