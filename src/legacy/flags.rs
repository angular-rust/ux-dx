use glib::translate::*;
use glib::{
    value::{FromValue, FromValueOptional, SetValue, Value},
    StaticType, Type,
};

bitflags! {
    pub struct BufferAccess: u32 {
        const READ = 1;
        const WRITE = 2;
        const READ_WRITE = 3;
    }
}

#[doc(hidden)]
impl ToGlib for BufferAccess {
    type GlibType = ffi::CoglBufferAccess;

    fn to_glib(&self) -> ffi::CoglBufferAccess {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglBufferAccess> for BufferAccess {
    fn from_glib(value: ffi::CoglBufferAccess) -> BufferAccess {
        BufferAccess::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct BufferBit: u32 {
        const COLOR = 1;
        const DEPTH = 2;
        const STENCIL = 4;
    }
}

#[doc(hidden)]
impl ToGlib for BufferBit {
    type GlibType = ffi::CoglBufferBit;

    fn to_glib(&self) -> ffi::CoglBufferBit {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglBufferBit> for BufferBit {
    fn from_glib(value: ffi::CoglBufferBit) -> BufferBit {
        BufferBit::from_bits_truncate(value)
    }
}

impl StaticType for BufferBit {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_buffer_bit_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for BufferBit {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for BufferBit {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for BufferBit {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct BufferMapHint: u32 {
        const DISCARD = 1;
        const DISCARD_RANGE = 2;
    }
}

#[doc(hidden)]
impl ToGlib for BufferMapHint {
    type GlibType = ffi::CoglBufferMapHint;

    fn to_glib(&self) -> ffi::CoglBufferMapHint {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglBufferMapHint> for BufferMapHint {
    fn from_glib(value: ffi::CoglBufferMapHint) -> BufferMapHint {
        BufferMapHint::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct BufferTarget: u32 {
        const WINDOW_BUFFER = 2;
        const OFFSCREEN_BUFFER = 4;
    }
}

#[doc(hidden)]
impl ToGlib for BufferTarget {
    type GlibType = ffi::CoglBufferTarget;

    fn to_glib(&self) -> ffi::CoglBufferTarget {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglBufferTarget> for BufferTarget {
    fn from_glib(value: ffi::CoglBufferTarget) -> BufferTarget {
        BufferTarget::from_bits_truncate(value)
    }
}

impl StaticType for BufferTarget {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_buffer_target_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for BufferTarget {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for BufferTarget {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for BufferTarget {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
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

#[doc(hidden)]
impl ToGlib for ColorMask {
    type GlibType = ffi::CoglColorMask;

    fn to_glib(&self) -> ffi::CoglColorMask {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglColorMask> for ColorMask {
    fn from_glib(value: ffi::CoglColorMask) -> ColorMask {
        ColorMask::from_bits_truncate(value)
    }
}

impl StaticType for ColorMask {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_color_mask_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ColorMask {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ColorMask {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for ColorMask {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
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

#[doc(hidden)]
impl ToGlib for FeatureFlags {
    type GlibType = ffi::CoglFeatureFlags;

    fn to_glib(&self) -> ffi::CoglFeatureFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglFeatureFlags> for FeatureFlags {
    fn from_glib(value: ffi::CoglFeatureFlags) -> FeatureFlags {
        FeatureFlags::from_bits_truncate(value)
    }
}

impl StaticType for FeatureFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_feature_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for FeatureFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for FeatureFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for FeatureFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct ReadPixelsFlags: u32 {
        const COLOR_BUFFER = 1;
    }
}

#[doc(hidden)]
impl ToGlib for ReadPixelsFlags {
    type GlibType = ffi::CoglReadPixelsFlags;

    fn to_glib(&self) -> ffi::CoglReadPixelsFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglReadPixelsFlags> for ReadPixelsFlags {
    fn from_glib(value: ffi::CoglReadPixelsFlags) -> ReadPixelsFlags {
        ReadPixelsFlags::from_bits_truncate(value)
    }
}

impl StaticType for ReadPixelsFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_read_pixels_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ReadPixelsFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ReadPixelsFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for ReadPixelsFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct RendererConstraint: u32 {
        const USES_X11 = 1;
        const USES_XLIB = 2;
        const USES_EGL = 4;
        const SUPPORTS_COGL_GLES2 = 8;
    }
}

#[doc(hidden)]
impl ToGlib for RendererConstraint {
    type GlibType = ffi::CoglRendererConstraint;

    fn to_glib(&self) -> ffi::CoglRendererConstraint {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglRendererConstraint> for RendererConstraint {
    fn from_glib(value: ffi::CoglRendererConstraint) -> RendererConstraint {
        RendererConstraint::from_bits_truncate(value)
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

#[doc(hidden)]
impl ToGlib for TextureFlags {
    type GlibType = ffi::CoglTextureFlags;

    fn to_glib(&self) -> ffi::CoglTextureFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::CoglTextureFlags> for TextureFlags {
    fn from_glib(value: ffi::CoglTextureFlags) -> TextureFlags {
        TextureFlags::from_bits_truncate(value)
    }
}

impl StaticType for TextureFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::cogl_texture_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for TextureFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for TextureFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for TextureFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}
