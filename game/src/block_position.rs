use bevy::prelude::*;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct BlockPos {
    value: IVec3,
}

impl Deref for BlockPos {
    type Target = IVec3;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<IVec3> for BlockPos {
    fn from(value: IVec3) -> Self {
        Self { value }
    }
}

// TODO implement an iterator that will iter all its neighbouring blocks

impl BlockPos {
    pub fn value(&self) -> IVec3 {
        self.value
    }

    pub fn front(&self) -> BlockPos {
        BlockPos::from(self.value + IVec3::Z)
    }

    pub fn back(&self) -> BlockPos {
        BlockPos::from(self.value + IVec3::NEG_Z)
    }

    pub(crate) fn neighbours(&self) -> impl Iterator<Item = BlockPos> {
        [
            self.value + IVec3::X,
            self.value + IVec3::NEG_X,
            self.value + IVec3::Z,
            self.value + IVec3::NEG_Z,
            self.value + IVec3::Y,
            self.value + IVec3::NEG_Y,
        ]
        .into_iter()
        .map(|p| p.into())
    }
}
