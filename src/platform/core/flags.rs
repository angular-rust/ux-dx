// BufferAccess:
// @BUFFER_ACCESS_READ: the buffer will be read
// @BUFFER_ACCESS_WRITE: the buffer will written to
// @BUFFER_ACCESS_READ_WRITE: the buffer will be used for both reading and
//   writing
//
// The access hints for buffer_set_update_hint()
//
// Since: 1.2
// Stability: unstable
bitflags! {
    pub struct BufferAccess: u32 {
        const READ = 1;
        const WRITE = 2;
        const READ_WRITE = 3;
    }
}

bitflags! {
    pub struct BufferBit: u32 {
        const COLOR = 1;
        const DEPTH = 2;
        const STENCIL = 4;
    }
}

bitflags! {
    pub struct BufferMapHint: u32 {
        const DISCARD = 1;
        const DISCARD_RANGE = 2;
    }
}

bitflags! {
    pub struct BufferTarget: u32 {
        const WINDOW_BUFFER = 2;
        const OFFSCREEN_BUFFER = 4;
    }
}

bitflags! {
    pub struct ColorMask: u32 {
        const NONE = 0;
        const RED = 1;
        const GREEN = 2;
        const BLUE = 4;
        const ALPHA = 8;
        const ALL = 15;
    }
}

impl Default for ColorMask {
    fn default() -> Self {
        Self::NONE
    }
}

bitflags! {
    pub struct FeatureFlags: u32 {
        const TEXTURE_RECTANGLE = 2;
        const TEXTURE_NPOT = 4;
        const TEXTURE_YUV = 8;
        const TEXTURE_READ_PIXELS = 16;
        const SHADERS_GLSL = 32;
        const OFFSCREEN = 64;
        const OFFSCREEN_MULTISAMPLE = 128;
        const OFFSCREEN_BLIT = 256;
        const FOUR_CLIP_PLANES = 512;
        const STENCIL_BUFFER = 1024;
        const VBOS = 2048;
        const PBOS = 4096;
        const UNSIGNED_INT_INDICES = 8192;
        const DEPTH_RANGE = 16384;
        const TEXTURE_NPOT_BASIC = 32768;
        const TEXTURE_NPOT_MIPMAP = 65536;
        const TEXTURE_NPOT_REPEAT = 131072;
        const POINT_SPRITE = 262144;
        const TEXTURE_3D = 524288;
        const SHADERS_ARBFP = 1048576;
        const MAP_BUFFER_FOR_READ = 2097152;
        const MAP_BUFFER_FOR_WRITE = 4194304;
        const ONSCREEN_MULTIPLE = 8388608;
        const DEPTH_TEXTURE = 16777216;
    }
}

bitflags! {
    pub struct ReadPixelsFlags: u32 {
        const COLOR_BUFFER = 1;
    }
}

bitflags! {
    pub struct RendererConstraint: u32 {
        const USES_X11 = 1;
        const USES_XLIB = 2;
        const USES_EGL = 4;
        const SUPPORTS_GLES2 = 8;
    }
}

bitflags! {
    pub struct TextureFlags: u32 {
        const NONE = 0;
        const NO_AUTO_MIPMAP = 1;
        const NO_SLICING = 2;
        const NO_ATLAS = 4;
    }
}
