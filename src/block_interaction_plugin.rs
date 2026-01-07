use bevy::prelude::*;

use crate::{
    BlockType, SelectedBlock,
    block_selection_plugin::HoveredBlockInfo,
    grid_plugin::{BlockChange, Grid, PlaceRequest, RemoveRequest, UpdateRequest},
};

pub struct BlockInteractionPlugin;

impl Plugin for BlockInteractionPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            Update,
            (request_place_selected_block, request_delete_hovered_block),
        );
    }
}

pub fn request_place_selected_block(
    mut commands: Commands,
    selected_block: Res<SelectedBlock>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    hovered_block: Res<HoveredBlockInfo>,
    grid: Res<Grid>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left)
        && let Some(position) = hovered_block.position
        && let Some(normal) = hovered_block.normal
    {
        if let Some(mut block_type) = selected_block.0 {
            if has_face(&block_type) {
                set_face(&mut block_type, normal);
            }

            commands.trigger(BlockChange::Place(PlaceRequest {
                position,
                normal,
                block_type,
            }));
        } else {
            let Some(block_data) = grid.get(position) else {
                return;
            };

            if is_button(&block_data)
                && is_pressed(&block_data)
                && let BlockType::StoneButton { on_side, ticks, .. } = block_data.block_type
            {
                commands.trigger(BlockChange::Update(UpdateRequest {
                    position,
                    block_type: BlockType::StoneButton {
                        pressed: true,
                        power: 15,
                        on_side,
                        ticks: 15,
                    },
                }));
            }
        }
    }
}

fn is_button(block_data: &&crate::BlockData) -> bool {
    match block_data.block_type {
        BlockType::StoneButton { .. } => true,
        _ => false,
    }
}

fn is_pressed(block_data: &&crate::BlockData) -> bool {
    match block_data.block_type {
        BlockType::StoneButton { pressed, .. } => !pressed,
        _ => false,
    }
}

fn set_face(block_type: &mut BlockType, normal: IVec3) {
    let face = face_from_normal(normal);

    match block_type {
        BlockType::RedStoneTorch { on_side, .. } => *on_side = face,
        _ => {}
    }
}

pub fn request_delete_hovered_block(
    mut commands: Commands,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    hovered_block: Res<HoveredBlockInfo>,
) {
    if mouse_buttons.just_pressed(MouseButton::Right)
        && let Some(position) = hovered_block.position
    {
        commands.trigger(BlockChange::Remove(RemoveRequest { position }));
    }
}

fn has_face(block_type: &BlockType) -> bool {
    matches!(block_type, BlockType::RedStoneTorch { .. })
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BlockFace {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

fn face_from_normal(n: IVec3) -> BlockFace {
    if n == IVec3::X {
        BlockFace::PosX
    } else if n == IVec3::NEG_X {
        BlockFace::NegX
    } else if n == IVec3::Y {
        BlockFace::PosY
    } else if n == IVec3::NEG_Y {
        BlockFace::NegY
    } else if n == IVec3::Z {
        BlockFace::PosZ
    } else {
        BlockFace::NegZ
    }
}
