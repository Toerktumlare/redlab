use bevy::prelude::*;

use crate::{
    RenderCtx, TextureAtlas,
    block_selection_plugin::{track_grid_cordinate, track_hovered_block, untrack_hovered_block},
    blocks::{ALL_DIRS, Block, BlockType, NeighbourUpdate, RecomputedResult, Renderable, Tickable},
    grid_plugin::Grid,
    meshes::{MeshId, MeshRegistry},
    redstone::NotifyDelay,
    render::Position,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RedStoneLamp {
    power: u8,
}

impl RedStoneLamp {
    fn resolve_power(&self, grid: &Grid, position: IVec3) -> u8 {
        let mut new_power = 0;
        for dir in ALL_DIRS {
            let neighbour_pos = position + dir;
            let Some(neighbour_block) = grid.get_blocktype(neighbour_pos) else {
                continue;
            };

            new_power = new_power.max(neighbour_block.strong_power_emitted_to(
                position,
                neighbour_pos,
                neighbour_block,
            ));

            new_power = new_power.max(
                neighbour_block
                    .weak_power_emitted(position, neighbour_pos, &BlockType::RedStoneLamp(*self))
                    .saturating_sub(1),
            );
        }
        new_power
    }
}

impl Block for RedStoneLamp {
    fn on_placement(&self, grid: &Grid, position: IVec3, _normal: IVec3) -> RecomputedResult<'_> {
        let Some(_) = grid.get(position) else {
            let power = self.resolve_power(grid, position);
            return RecomputedResult::Changed {
                new_block: Some(BlockType::RedStoneLamp(RedStoneLamp { power })),
                visual_update: true,
                self_tick: None,
                neighbor_tick: NeighbourUpdate::NONE,
            };
        };
        RecomputedResult::Unchanged
    }

    fn neighbor_changed(&self, grid: &Grid, position: IVec3) -> RecomputedResult<'_> {
        let None = grid.get(position) else {
            return RecomputedResult::Unchanged;
        };
        RecomputedResult::Changed {
            new_block: Some(BlockType::RedStoneLamp(*self)),
            visual_update: true,
            self_tick: Some(NotifyDelay::Immediate),
            neighbor_tick: NeighbourUpdate::DEFAULT,
        }
    }

    fn try_place(&self, _grid: &Grid, _position: IVec3) -> bool {
        true
    }

    fn power(&self) -> u8 {
        self.power
    }
}

impl Tickable for RedStoneLamp {
    fn on_tick(&self, grid: &Grid, position: IVec3) -> RecomputedResult<'_> {
        let mut new_power = 0;
        for dir in ALL_DIRS {
            let neighbour_pos = position + dir;
            let Some(neighbour_block) = grid.get_blocktype(neighbour_pos) else {
                continue;
            };

            new_power = new_power.max(neighbour_block.strong_power_emitted_to(
                position,
                neighbour_pos,
                neighbour_block,
            ));

            new_power = new_power.max(
                neighbour_block
                    .weak_power_emitted(position, neighbour_pos, &BlockType::RedStoneLamp(*self))
                    .saturating_sub(1),
            );
        }

        info!("Power resolved: {} for lamps", new_power);
        if self.power() != new_power {
            RecomputedResult::Changed {
                new_block: Some(BlockType::RedStoneLamp(RedStoneLamp { power: new_power })),
                visual_update: true,
                self_tick: Some(NotifyDelay::Immediate),
                neighbor_tick: NeighbourUpdate::DEFAULT,
            }
        } else {
            RecomputedResult::Unchanged
        }
    }
}

impl Renderable for RedStoneLamp {
    fn spawn(&self, ctx: &mut RenderCtx, position: IVec3) {
        let Some(block_type) = ctx.grid.get_blocktype(position) else {
            return;
        };

        let mesh = get_mesh(block_type, &ctx.mesh_registry);

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

        let mesh = get_mesh(block_type, &ctx.mesh_registry);

        ctx.commands.entity(entity).insert(Mesh3d(mesh.clone()));
    }
}

fn get_mesh<'a>(block_type: &'a BlockType, mesh_registry: &'a MeshRegistry) -> &'a Handle<Mesh> {
    if block_type.power() > 0 {
        mesh_registry.get(MeshId::RedStoneLampOn)
    } else {
        mesh_registry.get(MeshId::RedStoneLampOff)
    }
    .expect("Could not load RedStoneLamp Mesh from registry")
}
