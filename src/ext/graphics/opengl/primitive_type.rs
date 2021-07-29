use crate::gles::enums::*;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum PrimitiveType {
    Points,
    LineStrip,
    LineLoop,
    Lines,
    LineStripAdjacency,
    LinesAdjacency,
    TriangleStrip,
    TriangleFan,
    Triangles,
    TriangleStripAdjacency,
    TrianglesAdjacency,
    Patches,
}

impl PrimitiveType {
    pub(crate) fn to_flag(&self) -> u32 {
        match self {
            Self::Points => GL_POINTS,
            Self::LineStrip => GL_LINE_STRIP,
            Self::LineLoop => GL_LINE_LOOP,
            Self::Lines => GL_LINES,
            Self::LineStripAdjacency => GL_LINE_STRIP_ADJACENCY,
            Self::LinesAdjacency => GL_LINES_ADJACENCY,
            Self::TriangleStrip => GL_TRIANGLE_STRIP,
            Self::TriangleFan => GL_TRIANGLE_FAN,
            Self::Triangles => GL_TRIANGLES,
            Self::TriangleStripAdjacency => GL_TRIANGLE_STRIP_ADJACENCY,
            Self::TrianglesAdjacency => GL_TRIANGLES_ADJACENCY,
            Self::Patches => GL_PATCHES,
        }
    }
}
