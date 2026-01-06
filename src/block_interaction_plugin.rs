use bevy::prelude::*;

use crate::{
    BlockType, SelectedBlock,
    block_selection_plugin::HoveredBlockInfo,
    grid_plugin::{BlockChange, PlaceRequest, RemoveRequest},
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
) {
    if mouse_buttons.just_pressed(MouseButton::Left)
        && let Some(mut block_type) = selected_block.0
        && let Some(position) = hovered_block.position
        && let Some(normal) = hovered_block.normal
    {
        info!(
            "position: {}, normal: {}, block: {:?}",
            position, normal, block_type,
        );

        if has_face(&block_type) {
            set_face(&mut block_type, normal);
        }

        commands.trigger(BlockChange::Place(PlaceRequest {
            position,
            normal,
            block_type,
        }));
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
