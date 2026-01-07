use crate::meshes::{
    definitions::BlockDefinition,
    uv::{PIXEL, UvLayout},
};
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    platform::collections::HashMap,
    prelude::*,
};

pub mod definitions;
pub mod uv;

#[derive(Resource)]
pub struct MeshRegistry {
    handles: HashMap<MeshId, Handle<Mesh>>,
}

impl MeshRegistry {
    pub fn get(&self, mesh_id: MeshId) -> Option<&Handle<Mesh>> {
        self.handles.get(&mesh_id)
    }
}

pub fn setup_mesh_registry(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let definitions = vec![
        &BlockDefinition::REDSTONE_TORCH,
        &BlockDefinition::STANDARD_GRASS,
        &BlockDefinition::STANDARD_DIRT,
        &BlockDefinition::REDSTONE_LAMP_ON,
        &BlockDefinition::REDSTONE_LAMP_OFF,
        &BlockDefinition::REDSTONE_BLOCK,
        &BlockDefinition::STONE_BUTTON,
    ];

    let registry = build_hash_registry(definitions, &mut meshes);
    commands.insert_resource(registry);
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MeshId {
    RedstoneTorchStem,
    RedstoneTorchGlow,
    StandardGrass,
    StandardDirt,
    RedStoneBlock,
    RedStoneLampOn,
    RedStoneLampOff,
    StoneButton,
}

#[derive(Clone)]
pub(crate) struct PartMesh {
    pub size: Vec3,
    pub uvs: UvLayout,
}

#[derive(Component)]
pub struct BlockPart {
    pub part: MeshId,
    pub mesh: PartMesh,
}

fn build_hash_registry(
    definitions: Vec<&BlockDefinition>,
    meshes: &mut Assets<Mesh>,
) -> MeshRegistry {
    let mut handles = HashMap::new();

    for def in definitions {
        for part in def.parts {
            let handle = meshes.add(build_mesh_part(&part.mesh));
            handles.insert(part.part, handle);
        }
    }

    MeshRegistry { handles }
}

fn build_mesh_part(mesh: &PartMesh) -> Mesh {
    let hx = mesh.size.x * 1.0;
    let hy = mesh.size.y * 1.0;
    let hz = mesh.size.z * 1.0;

    #[rustfmt::skip]
    let face_positions = [
        [[-hx, -hy, hz], [hx, -hy, hz], [hx, hy, hz], [-hx, hy, hz]],       // North
        [[hx, -hy, -hz], [-hx, -hy, -hz], [-hx, hy, -hz], [hx, hy, -hz]],   // South
        [[hx, -hy, hz], [hx, -hy, -hz], [hx, hy, -hz], [hx, hy, hz]],       // East
        [[-hx, -hy, -hz], [-hx, -hy, hz], [-hx, hy, hz], [-hx, hy, -hz]],   // West
        [[-hx, hy, hz], [hx, hy, hz], [hx, hy, -hz], [-hx, hy, -hz]],       // Top
        [[-hx, -hy, -hz], [hx, -hy, -hz], [hx, -hy, hz], [-hx, -hy, hz]],   // Bottom
    ];

    let normals: Vec<[f32; 3]> = vec![
        // North
        Vec3::Z.to_array(),
        Vec3::Z.to_array(),
        Vec3::Z.to_array(),
        Vec3::Z.to_array(),
        // South
        Vec3::NEG_Z.to_array(),
        Vec3::NEG_Z.to_array(),
        Vec3::NEG_Z.to_array(),
        Vec3::NEG_Z.to_array(),
        // East
        Vec3::X.to_array(),
        Vec3::X.to_array(),
        Vec3::X.to_array(),
        Vec3::X.to_array(),
        // West
        Vec3::NEG_X.to_array(),
        Vec3::NEG_X.to_array(),
        Vec3::NEG_X.to_array(),
        Vec3::NEG_X.to_array(),
        // Top
        Vec3::Y.to_array(),
        Vec3::Y.to_array(),
        Vec3::Y.to_array(),
        Vec3::Y.to_array(),
        // Bottom
        Vec3::NEG_Y.to_array(),
        Vec3::NEG_Y.to_array(),
        Vec3::NEG_Y.to_array(),
        Vec3::NEG_Y.to_array(),
    ];

    let mut positions = Vec::with_capacity(4);
    let mut uvs = Vec::with_capacity(4);
    let mut indices = Vec::with_capacity(12);

    for face in 0..6 {
        let face_uvs = match mesh.uvs {
            UvLayout::Same(uv) => uv,
            UvLayout::PerFace(faces) => faces[face],
        };

        for i in 0..4 {
            positions.push(face_positions[face][i]);
            uvs.push((face_uvs[i] * PIXEL).to_array());
        }

        let v = (face * 4) as u32;
        indices.extend_from_slice(&[v, v + 1, v + 2, v, v + 2, v + 3]);
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
