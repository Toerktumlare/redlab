use bevy::prelude::*;

pub(crate) type FaceUvs = [Vec2; 4];

#[derive(Clone, Copy)]
pub(crate) enum UvLayout {
    Same(FaceUvs),
    PerFace([FaceUvs; 6]),
}

pub const PIXEL: f32 = 1.0 / 320.0;

pub(crate) const STANDARD_GRASS_BOTTOM: FaceUvs = [
    Vec2::new(32.0, 32.0),
    Vec2::new(63.0, 32.0),
    Vec2::new(63.0, 63.0),
    Vec2::new(32.0, 63.0),
];

pub(crate) const STANDARD_GRASS_TOP: FaceUvs = [
    Vec2::new(0.0, 32.0),
    Vec2::new(31.0, 32.0),
    Vec2::new(31.0, 63.0),
    Vec2::new(0.0, 63.0),
];

pub(crate) const STANDARD_GRASS_SIDES: FaceUvs = [
    Vec2::new(95.0, 63.0),
    Vec2::new(64.0, 63.0),
    Vec2::new(64.0, 32.0),
    Vec2::new(95.0, 32.0),
];

pub(crate) const REDSTONE_LAMP_ON: FaceUvs = [
    Vec2::new(64.0, 64.0),
    Vec2::new(96.0, 64.0),
    Vec2::new(96.0, 96.0),
    Vec2::new(64.0, 96.0),
];

pub(crate) const REDSTONE_LAMP_OFF: FaceUvs = [
    Vec2::new(32.0, 64.0),
    Vec2::new(64.0, 64.0),
    Vec2::new(64.0, 96.0),
    Vec2::new(32.0, 96.0),
];

pub(crate) const REDSTONE_TORCH_TOP: FaceUvs = [
    Vec2::new(110.0, 108.0),
    Vec2::new(114.0, 108.0),
    Vec2::new(114.0, 112.0),
    Vec2::new(110.0, 112.0),
];

pub(crate) const REDSTONE_TORCH_BOTTOM: FaceUvs = [
    Vec2::new(110.0, 124.0),
    Vec2::new(114.0, 124.0),
    Vec2::new(114.0, 128.0),
    Vec2::new(110.0, 128.0),
];

pub(crate) const REDSTONE_TORCH_FRONT: FaceUvs = [
    Vec2::new(110.0, 128.0),
    Vec2::new(114.0, 128.0),
    Vec2::new(114.0, 108.0),
    Vec2::new(110.0, 108.0),
];

pub(crate) const REDSTONE_TORCH_BACK: FaceUvs = [
    Vec2::new(110.0, 128.0),
    Vec2::new(114.0, 128.0),
    Vec2::new(114.0, 108.0),
    Vec2::new(110.0, 108.0),
];

pub(crate) const REDSTONE_TORCH_SIDES: FaceUvs = [
    Vec2::new(110.0, 128.0),
    Vec2::new(114.0, 128.0),
    Vec2::new(114.0, 108.0),
    Vec2::new(110.0, 108.0),
];

pub(crate) const REDSTONE_TORCH_TOP_OFF: FaceUvs = [
    Vec2::new(142.0, 108.0),
    Vec2::new(146.0, 108.0),
    Vec2::new(146.0, 112.0),
    Vec2::new(142.0, 112.0),
];

pub(crate) const REDSTONE_TORCH_BOTTOM_OFF: FaceUvs = [
    Vec2::new(142.0, 124.0),
    Vec2::new(146.0, 124.0),
    Vec2::new(146.0, 128.0),
    Vec2::new(148.0, 128.0),
];

pub(crate) const REDSTONE_TORCH_FRONT_OFF: FaceUvs = [
    Vec2::new(142.0, 128.0),
    Vec2::new(146.0, 128.0),
    Vec2::new(146.0, 108.0),
    Vec2::new(142.0, 108.0),
];

pub(crate) const REDSTONE_TORCH_BACK_OFF: FaceUvs = [
    Vec2::new(142.0, 128.0),
    Vec2::new(146.0, 128.0),
    Vec2::new(146.0, 108.0),
    Vec2::new(142.0, 108.0),
];

pub(crate) const REDSTONE_TORCH_SIDES_OFF: FaceUvs = [
    Vec2::new(142.0, 128.0),
    Vec2::new(146.0, 128.0),
    Vec2::new(146.0, 108.0),
    Vec2::new(142.0, 108.0),
];

pub(crate) const REDSTONE_TORCH_GLOW: FaceUvs = [
    Vec2::new(116.0, 114.0),
    Vec2::new(108.0, 114.0),
    Vec2::new(108.0, 106.0),
    Vec2::new(116.0, 106.0),
];

pub(crate) const REDSTONE_BLOCK: FaceUvs = [
    Vec2::new(0.0, 64.0),
    Vec2::new(32.0, 64.0),
    Vec2::new(32.0, 96.0),
    Vec2::new(0.0, 96.0),
];

pub(crate) const STANDARD_DIRT: FaceUvs = [
    Vec2::new(32.0, 32.0),
    Vec2::new(64.0, 32.0),
    Vec2::new(64.0, 64.0),
    Vec2::new(32.0, 64.0),
];

pub(crate) const STONE_BLOCK: FaceUvs = [
    Vec2::new(200.0, 32.0),
    Vec2::new(200.0, 40.0),
    Vec2::new(192.0, 40.0),
    Vec2::new(192.0, 32.0),
];
