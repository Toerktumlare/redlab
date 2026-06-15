#[cfg(test)]
mod tests;

use crate::{
    blocks::Block,
    grid::Grid,
    scheduler::{BlockEvent, ScheduleEvent, Scheduler, WorldEvent},
    time_machine::TimeMachine,
};

// #[derive(Debug)]
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
    }

    pub fn update(&mut self) {
        self.time_machine.record(self.tick);
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
            }
        }
        self.time_machine.end_tick();
    }

    pub fn start_recording(&mut self) {
        self.time_machine.start(self.tick);
    }

    pub fn stop_recording(&mut self) {
        self.time_machine.stop();
    }

    // wont work, we need old state and new state
    pub fn rewind(&mut self) {
        if let Some(ht) = self.time_machine.scrub_back() {
            for _e in ht.events.iter().rev() {
                // match e {
                //     WorldEvent::Block(e) => match *e {
                //         BlockEvent::Set(p, s) => self.grid.replace_block(p, s),
                //         BlockEvent::Update(p, s) => self.grid.replace_block(p, s),
                //         BlockEvent::Remove(p) => self.grid.remove_block(p),
                //     },
                //     WorldEvent::Schedule(ScheduleEvent::Tick(p)) => {
                //         self.scheduler.schedule_future(*p, self.tick);
                //     }
                // }
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
            BlockEvent::Set(block_pos, block_state) => {
                if let Some((pos, state)) = self.grid.replace_block(block_pos, block_state) {
                    self.time_machine
                        .add_event(WorldEvent::Block(BlockEvent::Set(pos, state)));
                }
            }
            BlockEvent::Update(block_pos, block_state) => {
                if let Some((pos, state)) = self.grid.replace_block(block_pos, block_state) {
                    self.time_machine
                        .add_event(WorldEvent::Block(BlockEvent::Set(pos, state)));
                }
            }
            BlockEvent::Remove(block_pos) => {
                if let Some((pos, _)) = self.grid.remove_block(block_pos) {
                    self.time_machine
                        .add_event(WorldEvent::Block(BlockEvent::Remove(pos)));
                }
            }
        };
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
                self.time_machine
                    .add_event(WorldEvent::Schedule(ScheduleEvent::Tick(pos)))
            }
        }
    }

    fn reverse(&mut self, arg: i32) {
        // let events = self.time_machine.get_events(arg);

        // loop that will apply onto the grid, not via update
    }
}
