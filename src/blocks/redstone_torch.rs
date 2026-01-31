use bevy::{prelude::*, render::render_resource::Face};

use crate::{
    TextureAtlas,
    block_selection_plugin::{track_grid_cordinate, track_hovered_block, untrack_hovered_block},
    blocks::{Block, BlockType, NeighbourUpdate, RecomputedResult, Renderable},
    grid_plugin::Grid,
    meshes::MeshId,
    redstone::NotifyDelay,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RedStoneTorch {
    power: u8,
    attached_face: IVec3,
}

impl Block for RedStoneTorch {
    fn on_placement(&self, grid: &Grid, position: IVec3, normal: IVec3) -> RecomputedResult<'_> {
        let Some(_) = grid.get(position) else {
            return RecomputedResult::Changed {
                new_block: Some(BlockType::RedStoneTorch(RedStoneTorch {
                    power: 0,
                    attached_face: normal,
                })),
                visual_update: true,
                self_tick: None,
                neighbor_tick: NeighbourUpdate::NONE,
            };
        };
        RecomputedResult::Unchanged
    }

    fn neighbor_changed(
        &self,
        grid: &crate::grid_plugin::Grid,
        position: IVec3,
    ) -> RecomputedResult<'_> {
        let Some(_) = grid.get(position) else {
            return RecomputedResult::Changed {
                new_block: Some(BlockType::RedStoneTorch(*self)),
                visual_update: true,
                self_tick: Some(NotifyDelay::Immediate),
                neighbor_tick: NeighbourUpdate::DEFAULT,
            };
        };

        RecomputedResult::Unchanged
    }

    fn try_place(&self, _grid: &crate::grid_plugin::Grid, _position: IVec3) -> bool {
        true
    }

    fn power(&self) -> u8 {
        self.power
    }
}

impl Renderable for RedStoneTorch {
    fn spawn(&self, ctx: &mut crate::RenderCtx, position: IVec3) {
        let texture = ctx.atlas.handles.get(&TextureAtlas::Blocks);

        let Some(block_type) = ctx.grid.get_blocktype(position) else {
            return;
        };

        let BlockType::RedStoneTorch(block) = block_type else {
            return;
        };

        let stem_mesh = ctx
            .mesh_registry
            .get(MeshId::RedstoneTorchStem)
            .expect("Could not load redstone torch stem");

        let glow_mesh = ctx
            .mesh_registry
            .get(MeshId::RedstoneTorchGlow)
            .expect("Could not load redstone torch glow");

        let transform = {
            let slant_pos = (25.0_f32).to_radians();
            let slant_neg = (-25.0_f32).to_radians();
            let distance = 0.37;
            match block.attached_face {
                IVec3::NEG_X => {
                    Transform::from_translation(position.as_vec3() - Vec3::X * distance)
                        .with_rotation(Quat::from_rotation_z(slant_neg))
                }
                IVec3::X => {
                    Transform::from_translation(position.as_vec3() - Vec3::NEG_X * distance)
                        .with_rotation(Quat::from_rotation_z(slant_pos))
                }
                IVec3::NEG_Z => {
                    Transform::from_translation(position.as_vec3() - Vec3::Z * distance)
                        .with_rotation(Quat::from_rotation_x(slant_pos))
                }
                IVec3::Z => {
                    Transform::from_translation(position.as_vec3() - Vec3::NEG_Z * distance)
                        .with_rotation(Quat::from_rotation_x(slant_neg))
                }
                _ => Transform::from_translation(position.as_vec3() - Vec3::Y * 0.25),
            }
        };

        let entity = ctx
            .commands
            .spawn((
                Name::new("RedstoneTorch"),
                Mesh3d(stem_mesh.clone()),
                MeshMaterial3d(ctx.materials.add(StandardMaterial {
                    base_color_texture: texture.cloned(),
                    perceptual_roughness: 1.0,
                    ..default()
                })),
                transform,
                Pickable {
                    is_hoverable: true,
                    ..default()
                },
                children![(
                    Name::new("RedstoneTorchGlow"),
                    Mesh3d(glow_mesh.clone()),
                    MeshMaterial3d(ctx.materials.add(StandardMaterial {
                        emissive: LinearRgba::new(5.0, 0.0, 0.0, 1.0),
                        base_color: Color::linear_rgb(0.5, 0.0, 0.0),
                        perceptual_roughness: 1.0,
                        cull_mode: Some(Face::Front),
                        unlit: true,
                        ..default()
                    })),
                    Pickable {
                        is_hoverable: true,
                        ..default()
                    },
                    Transform::from_translation(Vec3::Y * 0.25),
                )],
            ))
            .observe(track_hovered_block)
            .observe(track_grid_cordinate)
            .observe(untrack_hovered_block)
            .id();

        ctx.block_entities.entities.insert(position, entity);
    }

    fn update(&self, _ctx: &mut crate::RenderCtx, _entity: Entity, _position: IVec3) {
        todo!()
    }
}
