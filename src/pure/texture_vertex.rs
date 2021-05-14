#![allow(unused_imports)]
use super::Color;
use std::mem;

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct TextureVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub tx: f32,
    pub ty: f32,
    pub color: Color,
}
