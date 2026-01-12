use bevy::prelude::*;

use crate::{grid_plugin::Grid, redstone_connection_plugin::JunctionType};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Power {
    pub strong: u8,
    pub weak: u8,
}
impl Power {
    fn has_power(&self) -> bool {
        self.strong > 0 || self.weak > 0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BlockType {
    Air,
    StandardGrass {
        power: Power,
    },
    Dirt {
        power: Power,
    },
    RedStone,
    RedStoneLamp {
        powered: bool,
    },
    RedStoneTorch {
        on: bool,
        attached_face: IVec3,
    },
    Dust {
        shape: JunctionType,
        power: Power,
    },
    StoneButton {
        pressed: bool,
        attached_face: IVec3,
        ticks: u8,
    },
}

impl BlockType {
    pub fn is_insulator(&self) -> bool {
        matches!(
            &self,
            BlockType::RedStone
                | BlockType::Dust { .. }
                | BlockType::StandardGrass { .. }
                | BlockType::RedStoneTorch { .. }
        )
    }

    pub fn is_button(&self) -> bool {
        matches!(&self, BlockType::StoneButton { .. })
    }

    pub fn is_pressed(&self) -> bool {
        match &self {
            BlockType::StoneButton { pressed, .. } => *pressed,
            _ => false,
        }
    }

    pub fn has_face(&self) -> bool {
        matches!(
            &self,
            BlockType::RedStoneTorch { .. } | BlockType::StoneButton { .. }
        )
    }

    pub fn set_attached_face(&mut self, new_attached_face: IVec3) {
        match self {
            BlockType::RedStoneTorch { attached_face, .. } => *attached_face = -new_attached_face,
            BlockType::StoneButton { attached_face, .. } => *attached_face = -new_attached_face,
            _ => {}
        }
    }

    // if placing a solid block on the ground.
    //  is block under solid?
    // if placing a block on a wall
    //  is the wall solid?
    // if placing a block underneath something
    //  is the block above solid?
    //
    // is this block allowed to be placed using this normal?
    //  if so, is the block we are facing solid or not?
    pub fn can_be_placed(&self, position: IVec3, normal: IVec3, grid: &Res<Grid>) -> bool {
        let Some(block_data) = grid.get(position) else {
            return false;
        };

        let underneath = block_data.block_type;
        match &self {
            BlockType::RedStoneTorch { .. } => {
                if normal == IVec3::NEG_Y {
                    return false;
                }

                let Some(block_data) = grid.get(position) else {
                    return false;
                };
                block_data.block_type.is_solid()
            }
            BlockType::Dust { .. } => {
                if normal != IVec3::Y {
                    return false;
                }

                let Some(block_data) = grid.get(position) else {
                    return false;
                };
                block_data.block_type.is_solid()
            }
            BlockType::StoneButton { .. } => underneath.is_solid(),
            _ => underneath.is_solid(),
        }
    }

    pub fn is_solid(&self) -> bool {
        match &self {
            BlockType::Air => false,
            BlockType::StandardGrass { .. } => true,
            BlockType::Dirt { .. } => true,
            BlockType::RedStone => true,
            BlockType::RedStoneLamp { .. } => true,
            BlockType::RedStoneTorch { .. } => false,
            BlockType::Dust { .. } => false,
            BlockType::StoneButton { .. } => false,
        }
    }

    pub fn strong_power_emitted_to(
        &self,
        asking_pos: IVec3,
        emitting_pos: IVec3,
        block_type: &BlockType,
    ) -> u8 {
        match &self {
            BlockType::RedStone => 15,
            BlockType::StoneButton {
                pressed,
                attached_face,
                ticks,
            } => {
                if *pressed && *ticks > 0 {
                    if asking_pos == emitting_pos + attached_face {
                        15
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            BlockType::RedStoneTorch { on, .. } => {
                if *on {
                    if asking_pos == emitting_pos + IVec3::Y && block_type.is_solid() {
                        15
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    pub fn strong_power(&self) -> u8 {
        match &self {
            BlockType::StandardGrass { power } => power.strong,
            _ => 0,
        }
    }

    pub fn weak_power(&self) -> u8 {
        match &self {
            BlockType::Dust { power, .. } => power.weak,
            BlockType::StandardGrass { power } => power.weak,
            BlockType::Dirt { power } => power.weak,
            _ => 0,
        }
    }

    pub fn with_weak_power(&self, new_power: u8) -> BlockType {
        match self {
            BlockType::Dust { power, shape } => BlockType::Dust {
                power: Power {
                    strong: power.strong,
                    weak: new_power,
                },
                shape: *shape,
            },
            BlockType::StandardGrass { power } => BlockType::StandardGrass {
                power: Power {
                    strong: power.strong,
                    weak: new_power,
                },
            },
            _ => todo!(),
        }
    }

    pub fn weak_power_emitted(
        &self,
        asking_pos: IVec3,
        emitting_pos: IVec3,
        block_type: &BlockType,
    ) -> u8 {
        match &self {
            BlockType::Dust { power, .. } => power.weak,
            BlockType::RedStoneTorch { on, attached_face } => {
                if *on {
                    if asking_pos == emitting_pos + attached_face {
                        0
                    } else {
                        15
                    }
                } else {
                    0
                }
            }
            BlockType::StoneButton {
                pressed,
                attached_face,
                ticks,
            } => {
                if *pressed && *ticks > 0 {
                    if asking_pos == emitting_pos + attached_face || attached_face != &IVec3::NEG_Y
                    {
                        0
                    } else if matches!(block_type, BlockType::Dust { .. }) {
                        15
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            BlockType::StandardGrass { power } => {
                if matches!(block_type, BlockType::Dust { .. }) {
                    power.weak
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    pub fn is_powered(&self) -> bool {
        match &self {
            BlockType::StandardGrass { power } => power.has_power(),
            BlockType::Dust { power, .. } => power.has_power(),
            BlockType::Dirt { power, .. } => power.has_power(),
            BlockType::RedStoneLamp { powered, .. } => *powered,
            _ => false,
        }
    }

    pub(crate) fn is_emitter(&self) -> bool {
        matches!(
            self,
            BlockType::RedStone
                | BlockType::RedStoneTorch { .. }
                | BlockType::StoneButton { .. }
                | BlockType::Dust { .. }
        )
    }
}
