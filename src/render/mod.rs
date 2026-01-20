use bevy::platform::collections::{HashMap, HashSet};
use bevy::prelude::*;

use crate::blocks::BlockType;
use crate::grid_plugin::Grid;

mod debug;
mod drain;
mod dust;
mod redstone_block;
mod redstone_lamp;
mod renderer;
mod standard_blocks;

pub use debug::{debug_info, hovered_block, scheduler_info};
pub use drain::cleanup;
pub use dust::dust;
pub use redstone_block::redstone_block;
pub use redstone_lamp::redstone_lamp;
pub use renderer::renderer;
pub use standard_blocks::basic_blocks;

#[derive(Component, Debug, Clone, Copy)]
pub struct Position(pub IVec3);

#[derive(Component, Debug, Clone, Copy)]
pub struct Pressed(pub bool);

#[derive(Resource, Default)]
pub struct BlockEntities {
    pub entities: HashMap<IVec3, Entity>,
}

#[derive(Resource, Default)]
pub struct DirtyBlocks {
    pub positions: HashSet<IVec3>,
}

impl DirtyBlocks {
    pub fn mark(&mut self, position: IVec3) {
        self.positions.insert(position);
    }

    pub fn drain(&mut self) -> Vec<IVec3> {
        self.positions.drain().collect()
    }

    pub fn filter<'a, F>(
        &'a self,
        grid: &'a Grid,
        mut predicate: F,
    ) -> impl Iterator<Item = (IVec3, &'a BlockType)> + 'a
    where
        F: FnMut(&BlockType) -> bool + 'a,
    {
        self.positions
            .iter()
            .filter_map(|p| grid.get(*p).map(|data| (*p, &data.block_type)))
            .filter(move |(_, block_type)| predicate(block_type))
    }
}

#[derive(Resource, Default)]
pub struct DirtyRender {
    pub positions: HashSet<IVec3>,
}

impl DirtyRender {
    pub fn mark(&mut self, position: IVec3) {
        self.positions.insert(position);
    }

    pub fn drain(&mut self) -> Vec<IVec3> {
        self.positions.drain().collect()
    }

    pub fn filter<'a, F>(
        &'a self,
        grid: &'a Grid,
        mut predicate: F,
    ) -> impl Iterator<Item = (IVec3, &'a BlockType)> + 'a
    where
        F: FnMut(&BlockType) -> bool + 'a,
    {
        self.positions
            .iter()
            .filter_map(|p| grid.get(*p).map(|data| (*p, &data.block_type)))
            .filter(move |(_, block_type)| predicate(block_type))
    }
}

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<BlockEntities>()
            .init_resource::<DirtyBlocks>();
    }
}
