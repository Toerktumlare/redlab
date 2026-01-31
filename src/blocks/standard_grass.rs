use bevy::prelude::*;

use crate::{
    RenderCtx, TextureAtlas,
    block_selection_plugin::{track_grid_cordinate, track_hovered_block, untrack_hovered_block},
    blocks::{Block, BlockType, Dirt, NeighbourUpdate, RecomputedResult, Renderable, Tickable},
    grid_plugin::Grid,
    meshes::MeshId,
    redstone::NotifyDelay,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct StandardGrass {
    power: u8,
}

impl Block for StandardGrass {
    fn on_placement(&self, grid: &Grid, position: IVec3, _normal: IVec3) -> RecomputedResult<'_> {
        let Some(_) = grid.get(position) else {
            return RecomputedResult::Changed {
                new_block: Some(BlockType::StandardGrass(*self)),
                visual_update: true,
                self_tick: Some(NotifyDelay::Immediate),
                neighbor_tick: NeighbourUpdate::DEFAULT,
            };
        };
        RecomputedResult::Unchanged
    }

    fn neighbor_changed(&self, grid: &Grid, position: IVec3) -> RecomputedResult<'_> {
        let Some(_) = grid.get(position) else {
            let above = position + IVec3::Y;
            let Some(above_block_data) = grid.get(above) else {
                return RecomputedResult::Changed {
                    new_block: Some(BlockType::StandardGrass(*self)),
                    visual_update: true,
                    self_tick: Some(NotifyDelay::Immediate),
                    neighbor_tick: NeighbourUpdate::DEFAULT,
                };
            };

            if matches!(
                above_block_data.block_type,
                BlockType::StandardGrass(_) | BlockType::Dirt(_)
            ) {
                return RecomputedResult::Changed {
                    new_block: Some(BlockType::Dirt(Dirt {
                        power: self.power(),
                    })),
                    visual_update: true,
                    self_tick: None,
                    neighbor_tick: NeighbourUpdate::NONE,
                };
            };

            return RecomputedResult::Changed {
                new_block: Some(BlockType::StandardGrass(*self)),
                visual_update: true,
                self_tick: None,
                neighbor_tick: NeighbourUpdate::NONE,
            };
        };

        let above = position + IVec3::Y;
        let Some(block_data) = grid.get(above) else {
            return RecomputedResult::Unchanged;
        };

        if let BlockType::StandardGrass(_) = block_data.block_type {
            return RecomputedResult::Changed {
                new_block: Some(BlockType::Dirt(Dirt {
                    power: self.power(),
                })),
                visual_update: true,
                self_tick: Some(NotifyDelay::Immediate),
                neighbor_tick: NeighbourUpdate::NONE,
            };
        };

        if block_data.block_type.power() == self.power() {
            return RecomputedResult::Unchanged;
        }

        RecomputedResult::Unchanged
    }

    fn try_place(&self, _grid: &Grid, _position: IVec3) -> bool {
        true
    }

    fn power(&self) -> u8 {
        self.power
    }
}

impl Tickable for StandardGrass {
    fn on_tick(&self, _grid: &Grid, position: IVec3) -> RecomputedResult<'_> {
        info!("Grass: {} ticked", position);
        RecomputedResult::Unchanged
    }
}

impl Renderable for StandardGrass {
    fn spawn(&self, ctx: &mut RenderCtx, position: IVec3) {
        let mesh = ctx
            .mesh_registry
            .get(MeshId::StandardGrass)
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

    fn update(&self, _ctx: &mut RenderCtx, _entity: Entity, _position: IVec3) {}
}
