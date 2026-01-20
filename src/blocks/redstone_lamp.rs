use bevy::prelude::*;

use crate::{
    RenderCtx, TextureAtlas,
    block_selection_plugin::{track_grid_cordinate, track_hovered_block, untrack_hovered_block},
    blocks::{Block, BlockType, RecomputedResult, Renderable},
    grid_plugin::Grid,
    meshes::MeshId,
    render::Position,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RedStoneLamp {
    power: u8,
}

impl Block for RedStoneLamp {
    fn neighbor_changed(&self, grid: &Grid, position: IVec3) -> RecomputedResult {
        let None = grid.get(position) else {
            return RecomputedResult::Unchanged;
        };
        RecomputedResult::Changed {
            new_block: Some(BlockType::RedStoneLamp(*self)),
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

impl Renderable for RedStoneLamp {
    fn spawn(&self, ctx: &mut RenderCtx, position: IVec3) {
        let mesh = ctx
            .mesh_registry
            .get(MeshId::RedStoneLampOff)
            .expect("Could not load Redstone Block Mesh from registry");

        let texture = ctx.atlas.handles.get(&TextureAtlas::Blocks);
        let entity = ctx
            .commands
            .spawn((
                Name::new("RedstoneLamp"),
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
                Position(position),
            ))
            .observe(track_hovered_block)
            .observe(track_grid_cordinate)
            .observe(untrack_hovered_block)
            .id();

        ctx.block_entities.entities.insert(position, entity);
    }

    fn update(&self, ctx: &mut RenderCtx, entity: Entity, position: IVec3) {
        let Some(block_type) = ctx.grid.get_blocktype(position) else {
            return;
        };

        let mesh = if block_type.power() > 0 {
            ctx.mesh_registry.get(MeshId::RedStoneLampOn)
        } else {
            ctx.mesh_registry.get(MeshId::RedStoneLampOff)
        }
        .expect("Could not load RedStoneLamp Mesh from registry");

        ctx.commands.entity(entity).insert(Mesh3d(mesh.clone()));
    }
}
