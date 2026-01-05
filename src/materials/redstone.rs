use bevy::{platform::collections::HashMap, prelude::*};

use crate::TextureAtlas;
use crate::Textures;
use crate::redstone_connection_plugin::JunctionType;

static RED_BASE: f32 = 0.8;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum RedstoneTexture {
    Dot,
    Line,
}

impl From<JunctionType> for RedstoneTexture {
    fn from(value: JunctionType) -> Self {
        match value {
            JunctionType::Dot => RedstoneTexture::Dot,
            _ => RedstoneTexture::Line,
        }
    }
}

impl RedstoneTexture {
    pub const ALL: [RedstoneTexture; 2] = [RedstoneTexture::Dot, RedstoneTexture::Line];
}

#[derive(Resource)]
pub struct RedstoneColors {
    pub colors: Vec<(LinearRgba, Color)>,
}

impl Default for RedstoneColors {
    fn default() -> Self {
        let mut colors = Vec::with_capacity(16);

        for level in 0..=15 {
            let t = level as f32 / 15.0; // linear 0..1
            let t_emissive = t * t; // nonlinear for glow
            let t_base = 0.2 + 0.6 * t;
            colors.push((
                LinearRgba::new(5.0 * t_emissive, 0.0, 0.0, 1.0),
                Color::linear_rgb(RED_BASE * t_base, 0.0, 0.0),
            ));
        }

        RedstoneColors { colors }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum RedstonePower {
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
    P10,
    P11,
    P12,
    P13,
    P14,
    P15,
}

impl RedstonePower {
    pub const ALL: [RedstonePower; 16] = [
        RedstonePower::P0,
        RedstonePower::P1,
        RedstonePower::P2,
        RedstonePower::P3,
        RedstonePower::P4,
        RedstonePower::P5,
        RedstonePower::P6,
        RedstonePower::P7,
        RedstonePower::P8,
        RedstonePower::P9,
        RedstonePower::P10,
        RedstonePower::P11,
        RedstonePower::P12,
        RedstonePower::P13,
        RedstonePower::P14,
        RedstonePower::P15,
    ];
}

impl From<u8> for RedstonePower {
    fn from(n: u8) -> Self {
        match n {
            0 => RedstonePower::P0,
            1 => RedstonePower::P1,
            2 => RedstonePower::P2,
            3 => RedstonePower::P3,
            4 => RedstonePower::P4,
            5 => RedstonePower::P5,
            6 => RedstonePower::P6,
            7 => RedstonePower::P7,
            8 => RedstonePower::P8,
            9 => RedstonePower::P9,
            10 => RedstonePower::P10,
            11 => RedstonePower::P11,
            12 => RedstonePower::P12,
            13 => RedstonePower::P13,
            14 => RedstonePower::P14,
            _ => RedstonePower::P15,
        }
    }
}

#[derive(Resource)]
pub struct RedstoneMaterials {
    pub materials: HashMap<(RedstonePower, RedstoneTexture), Handle<StandardMaterial>>,
}

impl RedstoneMaterials {
    pub fn get(
        &self,
        texture: RedstoneTexture,
        power: RedstonePower,
    ) -> Option<Handle<StandardMaterial>> {
        self.materials.get(&(power, texture)).cloned()
    }
}

pub fn setup_redstone_materials(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    redstone_colors: Res<RedstoneColors>,
    textures: Res<Textures>,
) {
    let mut map = HashMap::new();

    for &texture in RedstoneTexture::ALL.iter() {
        for (i, &power) in RedstonePower::ALL.iter().enumerate() {
            let (emissive, base_color) = redstone_colors.colors[i];
            let mat = StandardMaterial {
                base_color_texture: Some(textures.handles[&TextureAtlas::Blocks].clone()),
                base_color,
                emissive,
                perceptual_roughness: 1.0,
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                ..default()
            };
            map.insert((power, texture), materials.add(mat));
        }
    }

    commands.insert_resource(RedstoneMaterials { materials: map });
}
