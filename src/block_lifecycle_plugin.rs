use bevy::{color::palettes::css::YELLOW, prelude::*};

use crate::{
    Block, Textures,
    block_selection_plugin::{track_grid_cordinate, track_hovered_block, untrack_hovered_block},
    cube::{Cube, CubeTextures, TileCoords},
};

#[derive(Event)]
pub struct PlaceBlockRequestEvent {
    pub position: IVec3,
    pub block: Block,
}

#[derive(Event)]
pub struct RemoveBlockRequestEvent {
    pub entity: Entity,
}

pub struct BlockLifecyclePlugin;

impl Plugin for BlockLifecyclePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_observer(handle_block_placement)
            .add_observer(handle_block_removal);
    }
}

pub fn handle_block_placement(
    event: On<PlaceBlockRequestEvent>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    texture_map: ResMut<Textures>,
) {
    let position = event.position;
    let block = event.block;
    let texture = texture_map.handles.get(&Block::StandardGrass);

    match block {
        Block::StandardGrass => {
            let textures = CubeTextures::new(
                Some(TileCoords::new(0, 1)),
                Some(TileCoords::new(1, 1)),
                Some(TileCoords::new(2, 1)),
                Some(TileCoords::new(3, 1)),
                Some(TileCoords::new(4, 1)),
                Some(TileCoords::new(5, 1)),
            );
            let mesh = Cube::new(textures);

            commands
                .spawn((
                    Mesh3d(meshes.add(mesh)),
                    MeshMaterial3d(materials.add(StandardMaterial {
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
                .observe(untrack_hovered_block);
        }
        Block::RedStone => {
            let textures = CubeTextures::new(
                Some(TileCoords::new(0, 2)),
                Some(TileCoords::new(0, 2)),
                Some(TileCoords::new(0, 2)),
                Some(TileCoords::new(0, 2)),
                Some(TileCoords::new(0, 2)),
                Some(TileCoords::new(0, 2)),
            );
            let mesh = Cube::new(textures);

            commands
                .spawn((
                    Mesh3d(meshes.add(mesh)),
                    MeshMaterial3d(materials.add(StandardMaterial {
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
                .observe(untrack_hovered_block);
        }
        Block::RedStoneLamp => {
            info!("redstonelamp");
            let textures = CubeTextures::new(
                Some(TileCoords::new(2, 2)),
                Some(TileCoords::new(2, 2)),
                Some(TileCoords::new(2, 2)),
                Some(TileCoords::new(2, 2)),
                Some(TileCoords::new(2, 2)),
                Some(TileCoords::new(2, 2)),
            );
            let mesh = Cube::new(textures);

            commands
                .spawn((
                    Mesh3d(meshes.add(mesh)),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: texture.cloned(),
                        perceptual_roughness: 1.0,
                        ..default()
                    })),
                    Pickable {
                        is_hoverable: true,
                        ..default()
                    },
                    Transform::from_translation(position.as_vec3()),
                    children![(PointLight {
                        intensity: 30_000.0,
                        color: YELLOW.into(),
                        shadows_enabled: true,
                        ..default()
                    },)],
                ))
                .observe(track_hovered_block)
                .observe(track_grid_cordinate)
                .observe(untrack_hovered_block);
        }
        _ => {}
    }
}

fn handle_block_removal(event: On<RemoveBlockRequestEvent>, mut commands: Commands) {
    let entity = event.entity;
    commands.entity(entity).despawn();
}
