#![allow(unused_imports)]
use std::mem;

pub enum DebugFlags {
    // DEBUG_SLICING,
    // DEBUG_OFFSCREEN,
    // DEBUG_DRAW,
    // DEBUG_PANGO,
    // DEBUG_RECTANGLES,
    // DEBUG_OBJECT,
    // DEBUG_BLEND_STRINGS,
    // DEBUG_DISABLE_BATCHING,
    // DEBUG_DISABLE_VBOS,
    // DEBUG_DISABLE_PBOS,
    // DEBUG_JOURNAL,
    // DEBUG_BATCHING,
    // DEBUG_DISABLE_SOFTWARE_TRANSFORM,
    // DEBUG_MATRICES,
    // DEBUG_ATLAS,
    // DEBUG_DUMP_ATLAS_IMAGE,
    // DEBUG_DISABLE_ATLAS,
    // DEBUG_DISABLE_SHARED_ATLAS,
    // DEBUG_OPENGL,
    // DEBUG_DISABLE_TEXTURING,
    // DEBUG_DISABLE_ARBFP,
    // DEBUG_DISABLE_FIXED,
    // DEBUG_DISABLE_GLSL,
    // DEBUG_SHOW_SOURCE,
    // DEBUG_DISABLE_BLENDING,
    // DEBUG_TEXTURE_PIXMAP,
    // DEBUG_BITMAP,
    // DEBUG_DISABLE_NPOT_TEXTURES,
    // DEBUG_WIREFRAME,
    // DEBUG_DISABLE_SOFTWARE_CLIP,
    // DEBUG_DISABLE_PROGRAM_CACHES,
    // DEBUG_DISABLE_FAST_READ_PIXEL,
    // DEBUG_CLIPPING,
    // DEBUG_WINSYS,
    // DEBUG_PERFORMANCE,

    // DEBUG_N_FLAGS
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DebugObjectTypeInfo<'a> {
    pub name: &'a str, // TODO: may be Option<String>, see gdk::WindowAttr
    pub instance_count: u64,
}
