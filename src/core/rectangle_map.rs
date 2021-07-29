#[derive(Default, Debug, Clone)]
pub struct RectangleMapEntry {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[derive(Debug, Clone)]
pub enum RectangleMapNodeType {
    Branch,
    FilledLeaf,
    EmptyLeaf,
}

impl Default for RectangleMapNodeType {
    fn default() -> Self {
        Self::EmptyLeaf
    }
}

#[derive(Default, Debug, Clone)]
pub struct RectangleMapNode {
    kind: RectangleMapNodeType,

    rectangle: RectangleMapEntry,

    largest_gap: u32,

    parent: Option<Box<RectangleMapNode>>,
    //   union
    //   {
    //     /* Fields used when this is a branch */
    //     struct
    //     {
    //       CoglRectangleMapNode *left;
    //       CoglRectangleMapNode *right;
    //     } branch;

    //     /* Field used when this is a filled leaf */
    //     void *data;
    //   } d;
}

#[derive(Default, Debug, Clone)]
pub struct RectangleMapStackEntry {
    // The node to search
    node: Option<RectangleMapNode>,
    // Index of next branch of this node to explore. Basically either 0
    // to go left or 1 to go right
    next_index: bool,
}
#[derive(Default, Debug, Clone)]
pub struct RectangleMap {
    root: Option<RectangleMapNode>,

    n_rectangles: u32,

    space_remaining: u32,
    //   value_destroy_func: GDestroyNotify,

    // Stack used for walking the structure. This is only used during
    // the lifetime of a single function call but it is kept here as an
    // optimisation to avoid reallocating it every time it is needed */
    //   stack: Option<Vec<>>,
}

impl RectangleMap {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn stack_push() {}

    pub fn stack_pop() {}

    pub fn stack_get_top() {}

    pub fn node_split_horizontally() {}

    pub fn node_split_vertically() {}

    pub fn verify_recursive() {}

    pub fn get_space_remaining_recursive() {}

    pub fn verify() {}

    pub fn add() {}

    pub fn remove() {}

    pub fn get_width() {}

    pub fn get_height() {}

    pub fn get_remaining_space() {}

    pub fn get_n_rectangles() {}

    pub fn internal_foreach() {}

    pub fn foreach_cb() {}

    pub fn foreach() {}

    pub fn free_cb() {}

    pub fn dump_image_cb() {}

    pub fn dump_image() {}
}

// pub struct RectangleMapForeachClosure {
//   CoglRectangleMapCallback callback;
//   void *data;
// }
