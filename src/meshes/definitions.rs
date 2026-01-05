use bevy::prelude::*;

use crate::meshes::{
    BlockPart, MeshId, PartMesh,
    uv::{
        REDSTONE_BLOCK, REDSTONE_LAMP_OFF, REDSTONE_LAMP_ON, REDSTONE_TORCH_BACK,
        REDSTONE_TORCH_BOTTOM, REDSTONE_TORCH_FRONT, REDSTONE_TORCH_GLOW, REDSTONE_TORCH_SIDES,
        REDSTONE_TORCH_TOP, STANDARD_DIRT, STANDARD_GRASS_BOTTOM, STANDARD_GRASS_SIDES,
        STANDARD_GRASS_TOP, UvLayout,
    },
};

pub struct BlockDefinition {
    pub parts: &'static [BlockPart],
}

impl BlockDefinition {
    pub const REDSTONE_TORCH: Self = Self {
        parts: &[
            BlockPart {
                part: MeshId::RedstoneTorchStem,
                mesh: PartMesh {
                    size: Vec3::new(0.06, 0.35, 0.06),
                    uvs: UvLayout::PerFace([
                        REDSTONE_TORCH_FRONT,
                        REDSTONE_TORCH_BACK,
                        REDSTONE_TORCH_SIDES,
                        REDSTONE_TORCH_SIDES,
                        REDSTONE_TORCH_TOP,
                        REDSTONE_TORCH_BOTTOM,
                    ]),
                },
            },
            BlockPart {
                part: MeshId::RedstoneTorchGlow,
                mesh: PartMesh {
                    size: Vec3::new(0.08, 0.08, 0.08),
                    uvs: UvLayout::Same(REDSTONE_TORCH_GLOW),
                },
            },
        ],
    };

    pub const STANDARD_GRASS: Self = Self {
        parts: &[BlockPart {
            part: MeshId::StandardGrass,
            mesh: PartMesh {
                size: Vec3::new(0.5, 0.5, 0.5),
                uvs: UvLayout::PerFace([
                    STANDARD_GRASS_SIDES,
                    STANDARD_GRASS_SIDES,
                    STANDARD_GRASS_SIDES,
                    STANDARD_GRASS_SIDES,
                    STANDARD_GRASS_TOP,
                    STANDARD_GRASS_BOTTOM,
                ]),
            },
        }],
    };

    pub const STANDARD_DIRT: Self = Self {
        parts: &[BlockPart {
            part: MeshId::StandardDirt,
            mesh: PartMesh {
                size: Vec3::new(0.5, 0.5, 0.5),
                uvs: UvLayout::Same(STANDARD_DIRT),
            },
        }],
    };

    pub const REDSTONE_LAMP_ON: Self = Self {
        parts: &[BlockPart {
            part: MeshId::RedStoneLampOn,
            mesh: PartMesh {
                size: Vec3::new(0.5, 0.5, 0.5),
                uvs: UvLayout::Same(REDSTONE_LAMP_ON),
            },
        }],
    };

    pub const REDSTONE_LAMP_OFF: Self = Self {
        parts: &[BlockPart {
            part: MeshId::RedStoneLampOff,
            mesh: PartMesh {
                size: Vec3::new(0.5, 0.5, 0.5),
                uvs: UvLayout::Same(REDSTONE_LAMP_OFF),
            },
        }],
    };

    pub const REDSTONE_BLOCK: Self = Self {
        parts: &[BlockPart {
            part: MeshId::RedStoneBlock,
            mesh: PartMesh {
                size: Vec3::new(0.5, 0.5, 0.5),
                uvs: UvLayout::Same(REDSTONE_BLOCK),
            },
        }],
    };
}
