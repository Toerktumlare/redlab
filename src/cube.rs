use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use crate::Block;

pub struct Cube;

pub struct CubeTextures {
    top: Option<TileCoords>,
    bottom: Option<TileCoords>,
    left: Option<TileCoords>,
    right: Option<TileCoords>,
    back: Option<TileCoords>,
    front: Option<TileCoords>,
}

impl CubeTextures {
    pub fn new(
        top: Option<TileCoords>,
        bottom: Option<TileCoords>,
        left: Option<TileCoords>,
        right: Option<TileCoords>,
        front: Option<TileCoords>,
        back: Option<TileCoords>,
    ) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
            front,
            back,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileCoords {
    pub x: u32,
    pub y: u32,
}

impl TileCoords {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

const ATLAS_SIZE_PX: f32 = 320.0;
const TILE_SIZE_PX: f32 = 32.0;
const TILE_NORMALIZED_SIZE: f32 = TILE_SIZE_PX / ATLAS_SIZE_PX;

enum CubeSide {
    Top,
    Bottom,
    Left,
    Right,
    Back,
    Forward,
}

impl CubeSide {
    pub fn get_data(&self, tile_coords: &TileCoords, cube_side: &CubeSide) -> SideData {
        let n = -0.5;
        let p = 0.5;

        let uvs = get_uvs(tile_coords, cube_side);

        match self {
            CubeSide::Top => SideData {
                positions: [[n, p, n], [p, p, n], [p, p, p], [n, p, p]],
                uvs,
                normals: [Vec3::Y.to_array(); 4],
            },
            CubeSide::Bottom => SideData {
                positions: [[n, n, n], [p, n, n], [p, n, p], [n, n, p]],
                uvs,
                normals: [Vec3::NEG_Y.to_array(); 4],
            },
            CubeSide::Right => SideData {
                positions: [[p, p, p], [p, p, n], [p, n, n], [p, n, p]],
                uvs,
                normals: [Vec3::X.to_array(); 4],
            },
            CubeSide::Left => SideData {
                positions: [[n, p, p], [n, p, n], [n, n, n], [n, n, p]],
                uvs,
                normals: [Vec3::NEG_X.to_array(); 4],
            },
            CubeSide::Back => SideData {
                positions: [[n, p, p], [p, p, p], [p, n, p], [n, n, p]],
                uvs,
                normals: [Vec3::Z.to_array(); 4],
            },
            CubeSide::Forward => SideData {
                positions: [[p, n, n], [n, n, n], [n, p, n], [p, p, n]],
                uvs,
                normals: [Vec3::NEG_Z.to_array(); 4],
            },
        }
    }
}

struct SideData {
    positions: [[f32; 3]; 4],
    uvs: UvCoords,
    normals: [[f32; 3]; 4],
}

type UvCoords = [[f32; 2]; 4];

fn get_uvs(tile_coords: &TileCoords, cube_side: &CubeSide) -> UvCoords {
    let x = tile_coords.x as f32;
    let y = tile_coords.y as f32;

    let tile_size = TILE_NORMALIZED_SIZE;

    // Map x and y to the normalized value of each tile beeing 0.1 out of 1.0
    let u_min = x * tile_size;
    let v_min = y * tile_size;

    // ask for the next tiles min value to get this tiles max values
    let u_max = (x + 1.0) * tile_size;
    let v_max = (y + 1.0) * tile_size;

    // Starting at top left, then top right, bottom right, bottom left
    let mut uvs = [
        [u_min, v_min],
        [u_max, v_min],
        [u_max, v_max],
        [u_min, v_max],
    ];

    match cube_side {
        CubeSide::Bottom | CubeSide::Forward => {
            uvs.reverse();
            uvs
        }
        _ => uvs,
    }
}

impl Cube {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(textures: CubeTextures) -> Mesh {
        let sides = [
            (CubeSide::Top, textures.top),
            (CubeSide::Bottom, textures.bottom),
            (CubeSide::Right, textures.right),
            (CubeSide::Left, textures.left),
            (CubeSide::Back, textures.back),
            (CubeSide::Forward, textures.front),
        ];

        let mut positions = Vec::with_capacity(24);
        let mut uvs = Vec::with_capacity(24);
        let mut normals = Vec::with_capacity(24);
        let mut indices = Vec::with_capacity(24);

        let mut vertex_count = 0;

        for (cube_side, opt_tile_coord) in sides.iter() {
            if let Some(tile_coord) = opt_tile_coord {
                let data = cube_side.get_data(tile_coord, cube_side);
                positions.extend(data.positions);
                uvs.extend(data.uvs);
                normals.extend(data.normals);

                match cube_side {
                    // Faces using Pattern A (N, N+3, N+1, N+1, N+3, N+2)
                    CubeSide::Top | CubeSide::Right | CubeSide::Back => {
                        indices.extend_from_slice(&[
                            vertex_count,
                            vertex_count + 3,
                            vertex_count + 1,
                            vertex_count + 1,
                            vertex_count + 3,
                            vertex_count + 2,
                        ]);
                    }
                    // Faces using Pattern B (N, N+1, N+3, N+1, N+2, N+3)
                    CubeSide::Bottom | CubeSide::Left | CubeSide::Forward => {
                        indices.extend_from_slice(&[
                            vertex_count,
                            vertex_count + 1,
                            vertex_count + 3,
                            vertex_count + 1,
                            vertex_count + 2,
                            vertex_count + 3,
                        ]);
                    }
                }
                vertex_count += 4;
            }
        }

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_indices(Indices::U32(indices))
    }

    pub fn get(block: Block) -> Mesh {
        let texture = match block {
            Block::StandardGrass => CubeTextures::new(
                Some(TileCoords::new(0, 1)),
                Some(TileCoords::new(1, 1)),
                Some(TileCoords::new(2, 1)),
                Some(TileCoords::new(3, 1)),
                Some(TileCoords::new(4, 1)),
                Some(TileCoords::new(5, 1)),
            ),
            Block::RedStone => CubeTextures::new(
                Some(TileCoords::new(0, 0)),
                Some(TileCoords::new(1, 0)),
                Some(TileCoords::new(2, 0)),
                Some(TileCoords::new(3, 0)),
                Some(TileCoords::new(4, 0)),
                Some(TileCoords::new(5, 0)),
            ),
            Block::Dust => todo!(),
            Block::RedStoneLamp => todo!(),
        };

        Cube::new(texture)
    }
}
