use bevy::prelude::*;

use crate::{
    Grid, RenderCtx, block_position::BlockPos, grid_plugin::BlockChangeQueue,
    redstone::NotifyDelay, render::DirtyBlocks,
};

mod dirt;
mod dust;
mod redstone_block;
mod redstone_lamp;
mod redstone_torch;
mod standard_grass;

pub use dirt::Dirt;
pub use dust::Dust;
pub use redstone_block::RedStone;
pub use redstone_lamp::RedStoneLamp;
pub use redstone_torch::RedStoneTorch;
pub use standard_grass::StandardGrass;

pub const ALL_DIRS: &[IVec3; 6] = &[
    IVec3::Y,
    IVec3::NEG_Y,
    IVec3::Z,
    IVec3::NEG_Z,
    IVec3::X,
    IVec3::NEG_X,
];

pub const DIRS: &[IVec3; 4] = &[IVec3::Z, IVec3::NEG_Z, IVec3::X, IVec3::NEG_X];

pub trait Block {
    fn on_placement(&self, grid: &Grid, position: IVec3, normal: IVec3) -> RecomputedResult<'_>;
    fn neighbor_changed(&self, grid: &Grid, position: IVec3) -> RecomputedResult<'_>;
    fn try_place(&self, grid: &Grid, position: IVec3) -> bool;
    fn on_remove(&self, _grid: &Grid, _position: &BlockPos, _queue: &mut BlockChangeQueue) {}
}

pub trait Tickable {
    fn on_tick(&self, grid: &Grid, position: IVec3) -> RecomputedResult<'_>;
    fn power(&self) -> u8;
}

pub trait Renderable {
    fn spawn(&self, ctx: &mut RenderCtx, position: IVec3);
    fn update(&self, ctx: &mut RenderCtx, entity: Entity, position: IVec3);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BlockType {
    Air,
    StandardGrass(StandardGrass),
    Dirt(Dirt),
    RedStone(RedStone),
    RedStoneLamp(RedStoneLamp),
    RedStoneTorch(RedStoneTorch),
    Dust(Dust),
    // StoneButton {
    //     pressed: bool,
    //     attached_face: IVec3,
    //     ticks: u8,
    // },
}

impl BlockType {
    pub fn strong_power_emitted_to(
        &self,
        asking_pos: IVec3,
        emitting_pos: IVec3,
        _block_type: &BlockType,
    ) -> u8 {
        match &self {
            BlockType::RedStone(_) => 15,
            BlockType::RedStoneTorch(RedStoneTorch { lit, .. }) => {
                if !lit {
                    return 0;
                }

                if asking_pos + IVec3::NEG_Y == emitting_pos {
                    15
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    pub fn weak_power_emitted(
        &self,
        asking_pos: IVec3,
        emitting_pos: IVec3,
        _block_type: &BlockType,
    ) -> u8 {
        match &self {
            BlockType::Dust(block) => block.power(),
            BlockType::RedStoneTorch(RedStoneTorch { attached_face, .. }) => {
                let attached_pos = emitting_pos - attached_face;
                if asking_pos == attached_pos { 0 } else { 15 }
            }
            _ => 0,
        }
    }
}

impl Block for BlockType {
    // pub fn is_insulator(&self) -> bool {
    //     matches!(
    //         self,
    //         BlockType::RedStone
    //             | BlockType::Dust { .. }
    //             | BlockType::StandardGrass { .. }
    //             | BlockType::RedStoneTorch { .. }
    //     )
    // }

    // pub fn is_button(&self) -> bool {
    //     matches!(&self, BlockType::StoneButton { .. })
    // }

    // pub fn is_pressed(&self) -> bool {
    //     match &self {
    //         BlockType::StoneButton { pressed, .. } => *pressed,
    //         _ => false,
    //     }
    // }

    // pub fn has_face(&self) -> bool {
    //     matches!(
    //         &self,
    //         BlockType::RedStoneTorch { .. } | BlockType::StoneButton { .. }
    //     )
    // }

    // pub fn set_attached_face(&mut self, new_attached_face: IVec3) {
    //     match self {
    //         BlockType::RedStoneTorch { attached_face, .. } => *attached_face = -new_attached_face,
    //         BlockType::StoneButton { attached_face, .. } => *attached_face = -new_attached_face,
    //         _ => {}
    //     }
    // }

    // // if placing a solid block on the ground.
    // //  is block under solid?
    // // if placing a block on a wall
    // //  is the wall solid?
    // // if placing a block underneath something
    // //  is the block above solid?
    // //
    // // is this block allowed to be placed using this normal?
    // //  if so, is the block we are facing solid or not?
    // pub fn can_be_placed(&self, position: IVec3, normal: IVec3, grid: &Res<Grid>) -> bool {
    //     let Some(block_data) = grid.get(position) else {
    //         return false;
    //     };

    //     let underneath = block_data.block_type;
    //     match &self {
    //         BlockType::RedStoneTorch { .. } => {
    //             if normal == IVec3::NEG_Y {
    //                 return false;
    //             }

    //             let Some(block_data) = grid.get(position) else {
    //                 return false;
    //             };
    //             block_data.block_type.is_solid()
    //         }
    //         BlockType::Dust { .. } => {
    //             if normal != IVec3::Y {
    //                 return false;
    //             }

    //             let Some(block_data) = grid.get(position) else {
    //                 return false;
    //             };
    //             block_data.block_type.is_solid()
    //         }
    //         BlockType::StoneButton { .. } => underneath.is_solid(),
    //         _ => underneath.is_solid(),
    //     }
    // }

    // pub fn is_solid(&self) -> bool {
    //     match &self {
    //         BlockType::Air => false,
    //         BlockType::StandardGrass { .. } => true,
    //         BlockType::Dirt { .. } => true,
    //         BlockType::RedStone => true,
    //         BlockType::RedStoneLamp { .. } => true,
    //         BlockType::RedStoneTorch { .. } => false,
    //         BlockType::Dust { .. } => false,
    //         BlockType::StoneButton { .. } => false,
    //     }
    // }

    // pub fn strong_power_emitted_to(
    //     &self,
    //     asking_pos: IVec3,
    //     emitting_pos: IVec3,
    //     block_type: &BlockType,
    // ) -> u8 {
    //     match &self {
    //         BlockType::RedStone { .. } => 15,
    //         // BlockType::StoneButton {
    //         //     pressed,
    //         //     attached_face,
    //         //     ticks,
    //         // } => {
    //         //     if *pressed && *ticks > 0 {
    //         //         if asking_pos == emitting_pos + attached_face {
    //         //             15
    //         //         } else {
    //         //             0
    //         //         }
    //         //     } else {
    //         //         0
    //         //     }
    //         // }
    //         // BlockType::RedStoneTorch { on, .. } => {
    //         //     if *on {
    //         //         if asking_pos == emitting_pos + IVec3::Y && block_type.is_solid() {
    //         //             0
    //         //         } else {
    //         //             0
    //         //         }
    //         //     } else {
    //         //         0
    //         //     }
    //         // }
    //         _ => 0,
    //     }
    // }

    // pub fn power(&self) -> u8 {
    //     match &self {
    //         BlockType::Dust { power, .. } => *power,
    //         BlockType::StandardGrass { power } => *power,
    //         BlockType::Dirt { power } => *power,
    //         _ => 0,
    //     }
    // }

    // pub fn with_power(&self, new_power: u8) -> BlockType {
    //     match self {
    //         BlockType::Dust { shape, .. } => BlockType::Dust {
    //             power: new_power,
    //             shape: *shape,
    //         },
    //         BlockType::StandardGrass { .. } => BlockType::StandardGrass { power: new_power },
    //         _ => todo!(),
    //     }
    // }

    // pub fn weak_power_emitted(
    //     &self,
    //     asking_pos: IVec3,
    //     emitting_pos: IVec3,
    //     block_type: &BlockType,
    // ) -> u8 {
    //     match &self {
    //         BlockType::Dust { power, .. } => *power,
    //         BlockType::RedStoneTorch { on, attached_face } => {
    //             if *on {
    //                 if asking_pos == emitting_pos + attached_face {
    //                     0
    //                 } else {
    //                     15
    //                 }
    //             } else {
    //                 0
    //             }
    //         }
    //         BlockType::StoneButton {
    //             pressed,
    //             attached_face,
    //             ticks,
    //         } => {
    //             if *pressed && *ticks > 0 {
    //                 if asking_pos == emitting_pos + attached_face || attached_face != &IVec3::NEG_Y
    //                 {
    //                     0
    //                 } else if matches!(block_type, BlockType::Dust { .. }) {
    //                     15
    //                 } else {
    //                     0
    //                 }
    //             } else {
    //                 0
    //             }
    //         }
    //         BlockType::StandardGrass { power } => {
    //             if matches!(block_type, BlockType::Dust { .. }) {
    //                 *power
    //             } else {
    //                 0
    //             }
    //         }
    //         _ => 0,
    //     }
    // }

    // pub fn is_powered(&self) -> bool {
    //     match &self {
    //         BlockType::StandardGrass { power } => *power > 0,
    //         BlockType::Dust { power, .. } => *power > 0,
    //         BlockType::Dirt { power, .. } => *power > 0,
    //         BlockType::RedStoneLamp { powered, .. } => *powered,
    //         _ => false,
    //     }
    // }

    // pub(crate) fn is_emitter(&self) -> bool {
    //     matches!(
    //         self,
    //         BlockType::RedStone
    //             | BlockType::RedStoneTorch { .. }
    //             | BlockType::StoneButton { .. }
    //             | BlockType::Dust { .. }
    //     )
    // }

    fn on_placement(&self, grid: &Grid, position: IVec3, normal: IVec3) -> RecomputedResult<'_> {
        match self {
            BlockType::Air => todo!(),
            BlockType::StandardGrass(block) => block.on_placement(grid, position, normal),
            BlockType::Dirt(block) => block.on_placement(grid, position, normal),
            BlockType::RedStone(block) => block.on_placement(grid, position, normal),
            BlockType::RedStoneLamp(block) => block.on_placement(grid, position, normal),
            BlockType::RedStoneTorch(block) => block.on_placement(grid, position, normal),
            BlockType::Dust(block) => block.on_placement(grid, position, normal),
        }
    }

    fn neighbor_changed(&self, grid: &Grid, position: IVec3) -> RecomputedResult<'_> {
        match self {
            BlockType::StandardGrass(block) => block.neighbor_changed(grid, position),
            BlockType::RedStone(block) => block.neighbor_changed(grid, position),
            BlockType::Dirt(block) => block.neighbor_changed(grid, position),
            BlockType::RedStoneLamp(block) => block.neighbor_changed(grid, position),
            BlockType::Dust(block) => block.neighbor_changed(grid, position),
            BlockType::RedStoneTorch(block) => block.neighbor_changed(grid, position),
            _ => todo!("recalculate has not been implemented for: {:?}", self),
        }
    }

    fn try_place(&self, grid: &Grid, position: IVec3) -> bool {
        match self {
            BlockType::StandardGrass(block) => block.try_place(grid, position),
            BlockType::Dirt(block) => block.try_place(grid, position),
            BlockType::RedStone(block) => block.try_place(grid, position),
            BlockType::RedStoneLamp(block) => block.try_place(grid, position),
            BlockType::Dust(block) => block.try_place(grid, position),
            BlockType::RedStoneTorch(block) => block.try_place(grid, position),
            _ => todo!("try_place has not been implemented for: {:?}", self),
        }
    }

    fn on_remove(&self, grid: &Grid, position: &BlockPos, queue: &mut BlockChangeQueue) {
        match self {
            BlockType::Dust(block) => block.on_remove(grid, position, queue),
            _ => {}
        }
    }
}

impl Tickable for BlockType {
    fn on_tick(&self, grid: &Grid, position: IVec3) -> RecomputedResult<'_> {
        match self {
            BlockType::RedStoneLamp(block) => block.on_tick(grid, position),
            BlockType::Dust(block) => block.on_tick(grid, position),
            _ => RecomputedResult::Unchanged,
        }
    }

    fn power(&self) -> u8 {
        match self {
            BlockType::RedStoneLamp(red_stone_lamp) => red_stone_lamp.power(),
            BlockType::Dust(dust) => dust.power(),
            BlockType::RedStoneTorch(block) => block.power(),
            _ => 0,
        }
    }
}

impl Renderable for BlockType {
    fn spawn(&self, ctx: &mut RenderCtx, position: IVec3) {
        match self {
            BlockType::Air => {}
            BlockType::StandardGrass(standard_grass) => standard_grass.spawn(ctx, position),
            BlockType::Dirt(dirt) => dirt.spawn(ctx, position),
            BlockType::RedStone(red_stone) => red_stone.spawn(ctx, position),
            BlockType::RedStoneLamp(red_stone_lamp) => red_stone_lamp.spawn(ctx, position),
            BlockType::Dust(dust) => dust.spawn(ctx, position),
            BlockType::RedStoneTorch(block) => block.spawn(ctx, position),
        }
    }

    fn update(&self, ctx: &mut RenderCtx, entity: Entity, position: IVec3) {
        match self {
            BlockType::Air => {}
            BlockType::StandardGrass(standard_grass) => {
                standard_grass.update(ctx, entity, position)
            }
            BlockType::Dirt(dirt) => dirt.update(ctx, entity, position),
            BlockType::RedStone(red_stone) => red_stone.update(ctx, entity, position),
            BlockType::RedStoneLamp(red_stone_lamp) => red_stone_lamp.update(ctx, entity, position),
            BlockType::Dust(dust) => dust.update(ctx, entity, position),
            BlockType::RedStoneTorch(block) => block.update(ctx, entity, position),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RecomputedResult<'a> {
    Changed {
        new_block: Option<BlockType>,
        visual_update: bool,
        self_tick: Option<NotifyDelay>,
        neighbor_tick: &'a [NeighbourUpdate],
    },
    Unchanged,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NeighbourUpdate {
    pub position: IVec3,
    pub notification: NotifyDelay,
}

impl NeighbourUpdate {
    pub const NONE: &[Self] = &[];
    pub const DEFAULT: &[Self] = &[
        NeighbourUpdate::new(IVec3::X, NotifyDelay::Immediate),
        NeighbourUpdate::new(IVec3::NEG_X, NotifyDelay::Immediate),
        NeighbourUpdate::new(IVec3::Z, NotifyDelay::Immediate),
        NeighbourUpdate::new(IVec3::NEG_Z, NotifyDelay::Immediate),
        NeighbourUpdate::new(IVec3::Y, NotifyDelay::Immediate),
        NeighbourUpdate::new(IVec3::NEG_Y, NotifyDelay::Immediate),
    ];

    pub const EXTENDED: &[Self] = &[
        NeighbourUpdate::new(IVec3::X, NotifyDelay::Immediate),
        NeighbourUpdate::new(IVec3::NEG_X, NotifyDelay::Immediate),
        NeighbourUpdate::new(IVec3::Z, NotifyDelay::Immediate),
        NeighbourUpdate::new(IVec3::NEG_Z, NotifyDelay::Immediate),
        NeighbourUpdate::new(IVec3::Y, NotifyDelay::Immediate),
        NeighbourUpdate::new(IVec3::NEG_Y, NotifyDelay::Immediate),
        NeighbourUpdate::new(IVec3::new(1, 1, 0), NotifyDelay::Immediate),
        NeighbourUpdate::new(IVec3::new(-1, 1, 0), NotifyDelay::Immediate),
        NeighbourUpdate::new(IVec3::new(0, 1, 1), NotifyDelay::Immediate),
        NeighbourUpdate::new(IVec3::new(0, 1, -1), NotifyDelay::Immediate),
        NeighbourUpdate::new(IVec3::new(1, -1, 0), NotifyDelay::Immediate),
        NeighbourUpdate::new(IVec3::new(-1, -1, 0), NotifyDelay::Immediate),
        NeighbourUpdate::new(IVec3::new(0, -1, 1), NotifyDelay::Immediate),
        NeighbourUpdate::new(IVec3::new(0, -1, -1), NotifyDelay::Immediate),
    ];

    pub const fn new(position: IVec3, notification: NotifyDelay) -> Self {
        Self {
            position,
            notification,
        }
    }
}
