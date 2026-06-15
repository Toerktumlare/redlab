use crate::{
    blocks::Block,
    grid::Grid,
    scheduler::{BlockEvent, ScheduleEvent, Scheduler, WorldEvent},
    time_machine::TimeMachine,
};

#[derive(Debug)]
pub struct Registry {
    blocks: Vec<Box<dyn Block>>,
}
impl Registry {
    pub fn new(max_blocks: usize) -> Self {
        let blocks: Vec<Box<dyn Block>> = Vec::with_capacity(max_blocks);
        Self { blocks }
    }

    #[inline]
    pub fn register<T>(&mut self, block: T) -> u16
    where
        T: Block + 'static,
    {
        self.blocks.push(Box::new(block));
        (self.blocks.len() - 1) as u16
    }

    #[inline]
    pub fn get(&self, id: u16) -> Option<&dyn Block> {
        self.blocks.get(id as usize).map(|b| b.as_ref())
    }
}

pub struct World {
    tick: u64,
    registry: Registry,
    grid: Grid,
    scheduler: Scheduler,
    time_machine: TimeMachine,
}

impl World {
    pub fn new(grid: Grid, registry: Registry, scheduler: Scheduler) -> Self {
        Self {
            tick: 0,
            registry,
            grid,
            scheduler,
            time_machine: TimeMachine::new(1),
        }
    }

    pub fn tick(&mut self) {
        self.tick += 1;
        self.scheduler.advance(self.tick);
        self.time_machine.record(self.tick);
    }

    pub fn update(&mut self) {
        let mut safety_counter = 0;
        const MAX_ITERATIONS: u32 = 100_000;

        while !self.scheduler.immediate.is_empty() {
            safety_counter += 1;
            if safety_counter > MAX_ITERATIONS {
                self.scheduler.immediate.clear();
                break;
            }

            let mut events = std::mem::take(&mut self.scheduler.immediate);
            for event in events.drain(..) {
                self.handle_event(event);
                self.time_machine.add_event(event);
            }
        }
        self.time_machine.end_tick();
    }

    pub fn start_recording(&mut self) {
        self.time_machine.start();
    }

    pub fn stop_recording(&mut self) {
        self.time_machine.stop();
    }

    // wont work, we need old state and new state
    pub fn rewind(&mut self) {
        if let Some(ht) = self.time_machine.scrub_back() {
            self.tick = ht.tick;
            for e in ht.events.iter().rev() {
                match e {
                    WorldEvent::Block(e) => match *e {
                        BlockEvent::Set(p, s) => self.grid.set_block(p, s),
                        BlockEvent::Update(p, s) => self.grid.set_block(p, s),
                        BlockEvent::Remove(p) => self.grid.remove_block(p),
                    },
                    WorldEvent::Schedule(ScheduleEvent::Tick(p)) => {
                        self.scheduler.schedule_future(*p, self.tick);
                    }
                }
            }
        }
    }

    fn handle_event(&mut self, event: WorldEvent) {
        match event {
            WorldEvent::Block(e) => self.apply_block_events(e),
            WorldEvent::Schedule(e) => self.apply_scheduled_events(e),
        }
    }

    // all of these need to return the old state when applying a new state to the grid
    pub fn apply_block_events(&mut self, event: BlockEvent) {
        match event {
            BlockEvent::Set(block_pos, block_state) => self.grid.set_block(block_pos, block_state),
            BlockEvent::Update(block_pos, block_state) => {
                self.grid.set_block(block_pos, block_state)
            }
            BlockEvent::Remove(block_pos) => self.grid.remove_block(block_pos),
        }
    }

    pub fn apply_scheduled_events(&mut self, event: ScheduleEvent) {
        match event {
            ScheduleEvent::Tick(pos) => {
                let state = self.grid.get_block(pos);
                if let Some(block) = self.registry.get(state.id())
                    && block.is_tickable()
                {
                    block.on_tick(&self.grid, pos, state, &mut self.scheduler)
                }
            }
        }
    }
}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        blocks::{AirBlock, Base, Tickable},
        grid::*,
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
        fn on_tick(
            &self,
            grid: &Grid,
            pos: BlockPos,
            state: BlockState,
            scheduler: &mut Scheduler,
        ) {
            let new_state = BlockState::new(2, 0); // ID 2
            scheduler.block_update(pos, new_state);
        }

        fn power(
            &self,
            grid: &Grid,
            pos: BlockPos,
            state: BlockState,
            askers_pos: BlockPos,
            schedule: &mut Scheduler,
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
    fn test_scheduled_event_lifecycle() {
        let mut world = setup();
        let pos = BlockPos::new(1, 1, 1);

        world.grid.set_block(pos, BlockState::new(0, 0));
        world.scheduler.schedule_future(pos, 5);

        for _ in 0..4 {
            world.tick();
            world.update();
        }

        assert_eq!(world.grid.get_block(pos).payload(), 0);

        world.tick();
        world.update();

        assert_eq!(world.grid.get_block(pos).id(), 2);
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

        world.grid.set_block(pos, BlockState::new(id as u32, 0));
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

        assert_eq!(world.time_machine.history[0].tick, 0);
        assert_eq!(world.time_machine.history[0].events.len(), 2);
        assert_eq!(world.time_machine.history[1].tick, 1);
        assert_eq!(world.time_machine.history[1].events.len(), 1);
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
            world.tick();
        }

        assert_eq!(world.time_machine.history[0].tick, 20);
        assert_eq!(world.time_machine.history[0].events.len(), 1);
    }

    #[test]
    fn should_record_scheduled_jobs() {
        // todo, do we want to see what was scheduled during each tick?
        // or do we just want to look at all events linearily?
    }

    #[test]
    fn test_scrub_backward_time() {
        // just apply tick
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
}
