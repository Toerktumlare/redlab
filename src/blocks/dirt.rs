use bevy::prelude::*;

use crate::{
    RenderCtx,
    blocks::{Block, BlockType, RecomputedResult, Renderable, Tickable},
    grid_plugin::Grid,
    meshes::MeshId,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Dirt {
    power: u8,
}

impl Block for Dirt {
    fn neighbor_changed(&self, grid: &Grid, position: IVec3) -> RecomputedResult {
        let None = grid.get(position) else {
            return RecomputedResult::Unchanged;
        };
        RecomputedResult::Changed {
            new_block: Some(BlockType::Dirt(*self)),
            visual_update: true,
            self_tick: None,
            neighbor_tick: None,
        }
    }

    fn try_place(&self, _grid: &Grid, _position: IVec3) -> bool {
        true
    }

    fn power(&self) -> u8 {
        self.power
    }
}

impl Tickable for Dirt {
    fn on_tick(&self, _grid: &Grid, _position: IVec3) -> RecomputedResult {
        RecomputedResult::Unchanged
    }
}

impl Renderable for Dirt {
    fn spawn(&self, _ctx: &mut RenderCtx, _position: IVec3) {}

    fn update(&self, ctx: &mut RenderCtx, entity: Entity, _position: IVec3) {
        let mesh = ctx
            .mesh_registry
            .get(MeshId::StandardDirt)
            .expect("Could not load Standard Dirt Mesh from registry");
        ctx.commands.entity(entity).insert((Mesh3d(mesh.clone()),));
    }
}
