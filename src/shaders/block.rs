use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct BlockMaterial {
    // Texture atlas
    #[texture(0)]
    #[sampler(1)]
    pub atlas: Handle<Image>,

    // UV offset inside atlas (tile_x, tile_y)
    #[uniform(2)]
    pub uv_offset: Vec2,

    // UV scale (tile size)
    #[uniform(3)]
    pub uv_scale: Vec2,
}

impl Material for BlockMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/block.wgsl".into()
    }
}
