use bevy::platform::collections::{HashMap, HashSet};
use bevy::prelude::*;

pub mod block_renderer;
pub mod debug;
pub mod redstone_renderer;

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
}

#[derive(Resource, Default)]
pub struct DirtyRedstone {
    pub positions: HashSet<IVec3>,
}

impl DirtyRedstone {
    pub fn mark(&mut self, position: IVec3) {
        self.positions.insert(position);
    }

    pub fn drain(&mut self) -> Vec<IVec3> {
        self.positions.drain().collect()
    }
}

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<BlockEntities>()
            .init_resource::<DirtyBlocks>()
            .init_resource::<DirtyRedstone>();
    }
}
