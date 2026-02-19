use bevy::prelude::*;

use crate::{
    RenderCtx, TextureAtlas,
    blocks::{Block, BlockType, NeighbourUpdate, RecomputedResult, Renderable},
    grid_plugin::Grid,
    interactions::{track_grid_cordinate, track_hovered_block, untrack_hovered_block},
    meshes::MeshId,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RedStone;

impl Block for RedStone {
    fn on_placement(&self, grid: &Grid, position: IVec3, _normal: IVec3) -> RecomputedResult<'_> {
        let Some(_) = grid.get(position) else {
            return RecomputedResult::Changed {
                new_block: Some(BlockType::RedStone(*self)),
                visual_update: true,
                self_tick: None,
                neighbor_tick: NeighbourUpdate::DEFAULT,
            };
        };
        RecomputedResult::Unchanged
    }

    fn neighbor_changed(&self, grid: &Grid, position: IVec3) -> RecomputedResult<'_> {
        let None = grid.get(position) else {
            return RecomputedResult::Unchanged;
        };
        RecomputedResult::Changed {
            new_block: Some(BlockType::RedStone(*self)),
            visual_update: true,
            self_tick: None,
            neighbor_tick: NeighbourUpdate::DEFAULT,
        }
    }

    fn try_place(&self, _grid: &Grid, _position: IVec3) -> bool {
        true
    }
}

impl Renderable for RedStone {
    fn spawn(&self, ctx: &mut RenderCtx, position: IVec3) {
        let mesh = ctx
            .mesh_registry
            .get(MeshId::RedStoneBlock)
            .expect("Could not load Redstone Block Mesh from registry");

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

    fn update(&self, _ctx: &mut RenderCtx, _entity: Entity, _position: IVec3) {}
}
