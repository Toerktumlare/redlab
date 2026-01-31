use bevy::prelude::*;

use crate::{
    RenderCtx, TextureAtlas,
    block_selection_plugin::{track_grid_cordinate, track_hovered_block, untrack_hovered_block},
    blocks::{Block, BlockType, NeighbourUpdate, RecomputedResult, Renderable, Tickable},
    grid_plugin::Grid,
    meshes::MeshId,
    redstone::NotifyDelay,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Dirt {
    pub power: u8,
}

impl Block for Dirt {
    fn on_placement(&self, grid: &Grid, position: IVec3, _normal: IVec3) -> RecomputedResult<'_> {
        let Some(_) = grid.get(position) else {
            return RecomputedResult::Changed {
                new_block: Some(BlockType::Dirt(*self)),
                visual_update: true,
                self_tick: Some(NotifyDelay::Immediate),
                neighbor_tick: NeighbourUpdate::DEFAULT,
            };
        };
        RecomputedResult::Unchanged
    }
    fn neighbor_changed(&self, _grid: &Grid, _position: IVec3) -> RecomputedResult<'_> {
        RecomputedResult::Unchanged
    }

    fn try_place(&self, _grid: &Grid, _position: IVec3) -> bool {
        true
    }

    fn power(&self) -> u8 {
        self.power
    }
}

impl Tickable for Dirt {
    fn on_tick(&self, _grid: &Grid, _position: IVec3) -> RecomputedResult<'_> {
        RecomputedResult::Unchanged
    }
}

impl Renderable for Dirt {
    fn spawn(&self, ctx: &mut RenderCtx, position: IVec3) {
        let mesh = ctx
            .mesh_registry
            .get(MeshId::StandardDirt)
            .expect("Could not load Standard Grass Mesh from registry");

        let texture = ctx.atlas.handles.get(&TextureAtlas::Blocks);

        let entity = ctx
            .commands
            .spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(ctx.materials.add(StandardMaterial {
                    base_color_texture: texture.cloned(),
                    perceptual_roughness: 1.0,
                    ..default()
                })),
                Transform::from_translation(position.as_vec3()),
                Pickable {
                    is_hoverable: true,
                    ..default()
                },
            ))
            .observe(track_hovered_block)
            .observe(track_grid_cordinate)
            .observe(untrack_hovered_block)
            .id();
        ctx.block_entities.entities.insert(position, entity);
    }

    fn update(&self, ctx: &mut RenderCtx, entity: Entity, _position: IVec3) {
        let mesh = ctx
            .mesh_registry
            .get(MeshId::StandardDirt)
            .expect("Could not load Standard Dirt Mesh from registry");
        ctx.commands.entity(entity).insert((Mesh3d(mesh.clone()),));
    }
}
