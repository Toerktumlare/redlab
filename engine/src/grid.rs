use std::{
    fmt::{self, Formatter},
    mem,
};

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct BlockPos(u64);

impl BlockPos {
    pub const X_BITS: u32 = 22;
    pub const Z_BITS: u32 = 22;
    pub const Y_BITS: u32 = 20;

    const X_MASK: u64 = (1u64 << 22) - 1;
    const Z_MASK: u64 = (1u64 << 22) - 1;
    const Y_MASK: u64 = (1u64 << 20) - 1;

    const CENTER_X: i32 = 1 << (Self::X_BITS - 1);
    const CENTER_Z: i32 = 1 << (Self::Z_BITS - 1);
    const CENTER_Y: i32 = 1 << (Self::Y_BITS - 1);

    const Z_SHIFT_VAL: u64 = 1u64 << Self::X_BITS;
    const Y_SHIFT_VAL: u64 = 1u64 << (Self::X_BITS + Self::Z_BITS);

    pub fn new(x: i32, y: i32, z: i32) -> Self {
        let x_internal = (x + Self::CENTER_X).clamp(0, Self::X_MASK as i32) as u64;
        let z_internal = (z + Self::CENTER_Z).clamp(0, Self::Z_MASK as i32) as u64;
        let y_internal = (y + Self::CENTER_Y).clamp(0, Self::Y_MASK as i32) as u64;
        Self(
            x_internal
                | (z_internal << Self::X_BITS)
                | (y_internal << (Self::X_BITS + Self::Z_BITS)),
        )
    }

    pub fn unpack(&self) -> (i32, i32, i32) {
        let x = (self.0 & Self::X_MASK) as i32 - Self::CENTER_X;
        let z = ((self.0 >> Self::X_BITS) & Self::Z_MASK) as i32 - Self::CENTER_Z;
        let y = ((self.0 >> (Self::X_BITS + Self::Z_BITS)) & Self::Y_MASK) as i32 - Self::CENTER_Y;
        (x, y, z)
    }

    #[inline]
    pub fn north(&self) -> Self {
        Self(self.0 + Self::Z_SHIFT_VAL)
    }

    #[inline]
    pub fn south(&self) -> Self {
        Self(self.0 - Self::Z_SHIFT_VAL)
    }

    #[inline]
    pub fn east(&self) -> Self {
        Self(self.0 + 1)
    }

    #[inline]
    pub fn west(&self) -> Self {
        Self(self.0 - 1)
    }

    #[inline]
    pub fn up(&self) -> Self {
        Self(self.0 + Self::Y_SHIFT_VAL)
    }

    #[inline]
    pub fn down(&self) -> Self {
        Self(self.0 - Self::Y_SHIFT_VAL)
    }
}

impl fmt::Debug for BlockPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x, y, z) = self.unpack();
        f.debug_struct("BlockPos")
            .field("x", &x)
            .field("y", &y)
            .field("z", &z)
            .field("raw", &format_args!("{:#018X}", self.0)) // Shows 64-bit hex
            .finish()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct BlockState(u32);

impl BlockState {
    const ID_MASK: u32 = 0x3FF;
    const PAYLOAD: u32 = 10;

    pub fn new(id: u32, payload: u32) -> Self {
        Self((id & Self::ID_MASK) | (payload << Self::PAYLOAD))
    }

    #[inline]
    pub fn id(&self) -> u16 {
        (self.0 & Self::ID_MASK) as u16
    }

    #[inline]
    pub fn payload(&self) -> u32 {
        self.0 >> Self::PAYLOAD
    }

    #[inline]
    pub fn raw(&self) -> u32 {
        self.0
    }
}

impl std::fmt::Debug for BlockState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ID: {} | Bits: {:022b}", self.id(), self.payload())
    }
}

#[derive(Debug, Default)]
pub struct Grid {
    blocks: Vec<BlockState>,
    width: i32,
    height: i32,
    depth: i32,
    hw: i32,
    hh: i32,
    hd: i32,
}

impl Grid {
    pub fn new(width: i32, height: i32, depth: i32) -> Self {
        Self {
            blocks: vec![BlockState::default(); (width * height * depth) as usize],
            width,
            height,
            depth,
            hw: width / 2,
            hh: height / 2,
            hd: depth / 2,
        }
    }

    fn pos_to_index(&self, pos: BlockPos) -> Option<usize> {
        let (x, y, z) = pos.unpack();
        let lx = x + self.hw;
        let ly = y + self.hh;
        let lz = z + self.hd;

        if lx < 0 || lx >= self.width || ly < 0 || ly >= self.height || lz < 0 || lz >= self.depth {
            return None;
        }
        Some((lx + (lz * self.width) + (ly * self.width * self.width)) as usize)
    }

    pub fn get_block(&self, pos: BlockPos) -> BlockState {
        self.pos_to_index(pos)
            .map(|i| self.blocks[i])
            .unwrap_or_default()
    }

    pub fn replace_block(
        &mut self,
        pos: BlockPos,
        data: BlockState,
    ) -> Option<(BlockPos, BlockState)> {
        if let Some(i) = self.pos_to_index(pos) {
            let old = mem::replace(&mut self.blocks[i], data);
            return Some((pos, old));
        }
        None
    }

    pub fn remove_block(&mut self, pos: BlockPos) -> Option<(BlockPos, BlockState)> {
        if let Some(i) = self.pos_to_index(pos) {
            let old = mem::replace(&mut self.blocks[i], BlockState(0));
            return Some((pos, old));
        }
        None
    }
}
