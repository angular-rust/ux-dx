use cgmath::Point2;
use std::{fmt, rc::Rc};

use crate::engine::d2::{
    animation::AnimatedFloat,
    input::{EventSource, PointerEvent},
    math::{Math, Matrix, Rectangle},
    util::{BitSets, Disposable, Listener2, Signal1, SignalConnection},
    Component, Director, Entity, EntityManager, System,
};

use super::{BlendMode, Graphics};

#[derive(Default, Clone)]
pub struct Sprite {
    pub component: Component,
    /// X position, in pixels.
    pub x: AnimatedFloat,

    /// Y position, in pixels.
    pub y: AnimatedFloat,

    /// Rotation angle, in degrees.
    pub rotation: AnimatedFloat,

    /// Horizontal scale factor.
    pub scale_x: AnimatedFloat,

    /// Vertical scale factor.
    pub scale_y: AnimatedFloat,

    /// The X position of this sprite's anchor point. Local transformations are applied relative to
    /// this point.
    pub anchor_x: AnimatedFloat,

    /// The Y position of this sprite's anchor point. Local transformations are applied relative to
    /// this point.
    pub anchor_y: AnimatedFloat,

    /// The alpha (opacity) of this sprite, between 0 (invisible) and 1 (fully opaque).
    pub alpha: AnimatedFloat,

    /// The blend mode used to draw this sprite, or None to use its parent's blend mode.
    pub blend_mode: BlendMode,

    /// The scissor rectangle used for clipping/masking, in the local coordinate system. The scissor
    /// rectangle affects both rendering and hit testing, and applies to this sprite and all
    /// children. Rectangles with negative width or height are OK.
    ///  *
    /// __WARNING__: When using scissor testing, this sprite (and its parents) must not be rotated.
    /// The scissor rectangle must be axis-aligned when converted to screen coordinates.
    pub scissor: Option<Rectangle>,

    /// Whether this sprite should be drawn. Invisible sprites do not receive pointer events.
    pub visible: bool,

    /// Whether this sprite's position will be rounded to the nearest whole pixel when rendering, for
    /// crisper images. Defaults to true. This can be disabled for smoother animation, at the risk of
    /// fuzziness when an image lies on a subpixel.
    pub pixel_snapping: bool,

    /// Whether this sprite or any children should receive pointer events. Defaults to true.
    pub pointer_enabled: bool,

    // static
    // scratch_point: Point2<f32>, // = Point::new(), // i32
    pub(crate) local_matrix: Matrix,

    view_matrix: Matrix,
    view_matrix_update_count: i32,
    parent_view_matrix_update_count: i32,
    sin_cache: f32,
    cos_cache: f32,

    /// Emitted when the pointer is pressed down over this sprite.
    pointer_down: Option<Signal1<PointerEvent>>,
    /// Emitted when the pointer is moved over this sprite.
    pointer_move: Option<Signal1<PointerEvent>>,
    /// Emitted when the pointer is raised over this sprite.
    pointer_up: Option<Signal1<PointerEvent>>,
    /// Emitted when the pointer is moved inside of this sprite.
    pointer_in: Option<Signal1<PointerEvent>>,
    /// Emitted when the pointer is moved outside of this sprite. If the pointer is a touch, also
    /// emitted when the touch is lifted from this sprite.
    pointer_out: Option<Signal1<PointerEvent>>,

    hover_connection: Option<SignalConnection>,
}

impl Sprite {
    // Component flags
    pub const VISIBLE: u32 = Component::NEXT_FLAG << 0;
    pub const POINTER_ENABLED: u32 = Component::NEXT_FLAG << 1;
    pub const LOCAL_MATRIX_DIRTY: u32 = Component::NEXT_FLAG << 2;
    pub const VIEW_MATRIX_DIRTY: u32 = Component::NEXT_FLAG << 3;
    pub const PIXEL_SNAPPING: u32 = Component::NEXT_FLAG << 4;
    pub const HOVERING: u32 = Component::NEXT_FLAG << 5;
    pub const ROTATION_DIRTY: u32 = Component::NEXT_FLAG << 6;
    pub const NEXT_FLAG: u32 = Component::NEXT_FLAG << 7; // Must be last!

    pub fn new() -> Self {
        let flags = Self::VISIBLE
            | Self::POINTER_ENABLED
            | Self::VIEW_MATRIX_DIRTY
            | Self::PIXEL_SNAPPING
            | Self::ROTATION_DIRTY;

        let local_matrix = Matrix::new();

        let dirty_matrix: Option<Listener2<f32, f32>> = Some(Rc::new(|_, _| {
            // TODO: DV
            // flags = flags.add(Self::LOCAL_MATRIX_DIRTY | Self::VIEW_MATRIX_DIRTY);
        }));
        let x = AnimatedFloat::new(0.0, dirty_matrix.clone());
        let y = AnimatedFloat::new(0.0, dirty_matrix.clone());
        let rotation = AnimatedFloat::new(
            0.0,
            Some(Rc::new(move |_, _| {
                // flags = flags.add(Self::LOCAL_MATRIX_DIRTY | Self::VIEW_MATRIX_DIRTY | Self::ROTATION_DIRTY);
            })),
        );
        let scale_x = AnimatedFloat::new(1.0, dirty_matrix.clone());
        let scale_y = AnimatedFloat::new(1.0, dirty_matrix.clone());
        let anchor_x = AnimatedFloat::new(0.0, dirty_matrix.clone());
        let anchor_y = AnimatedFloat::new(0.0, dirty_matrix.clone());

        let alpha = AnimatedFloat::new(1.0, None);
        unimplemented!()
    }

    /// Search for a sprite in the entity hierarchy lying under the given point, in local
    /// coordinates. Ignores sprites that are invisible or not pointerEnabled during traversal.
    /// Returns None if neither the entity or its children contain a sprite under the given point.
    //  static
    pub fn hit_test(entity: Entity, x: f32, y: f32) -> Option<Sprite> {
        let sprite = EntityManager::<Sprite>::get(&entity);
        // if let Some(sprite) = *sprite {
        //     if !sprite.inner.flags.contains_all(Self::VISIBLE | Self::POINTER_ENABLED) {
        //         return None; // Prune invisible or non-interactive subtrees
        //     }

        //     // if sprite.get_local_matrix().inverse_transform(x, y, self.scratch_point) {
        //     //     x = self.scratch_point.x;
        //     //     y = self.scratch_point.y;
        //     // }

        //     // let scissor = sprite.scissor;
        //     // if scissor.is_some() && !scissor.contains(x, y) {
        //     //     return None; // Prune if outside the scissor rectangle
        //     // }
        //     unimplemented!()
        // }

        // // Hit test all children, front to back
        // let result = Self::hit_test_backwards(entity.first_child, x, y);
        // if result.is_some() {
        //     return result;
        // }

        // // Finally, if we got this far, hit test the actual sprite
        // if sprite.is_some() && sprite.contains_local(x, y) {
        //     sprite
        // } else {
        //     None
        // };
        unimplemented!()
    }

    /// Calculate the bounding box of an entity hierarchy. Returns the smallest rectangle in local
    /// coordinates that fully encloses all child sprites.
    //  static
    pub fn bounds(entity: Entity, result: Option<Rectangle>) -> Rectangle {
        let mut result = result.clone().unwrap_or_default();

        // The width and height of this rectangle are hijacked to store the bottom right corner
        result.set(
            Math::FLOAT_MAX,
            Math::FLOAT_MAX,
            Math::FLOAT_MIN,
            Math::FLOAT_MIN,
        );
        Self::bounds_impl(&entity, None, &mut result);

        // Convert back to a true width and height
        result.width -= result.x;
        result.height -= result.y;

        result
    }

    /// Renders an entity hierarchy to the given Graphics.
    //  static
    pub fn render(entity: &Entity, gfx: &Box<dyn Graphics>) {
        // Render this entity's sprite
        let mut sprite = EntityManager::<Sprite>::get(entity);
        if let Some(ref mut sprite) = sprite {
            let alpha = sprite.alpha.get();
            if !sprite.visible || alpha <= 0.0 {
                return; // Prune traversal, this sprite and all children are invisible
            }

            gfx.save();
            if alpha < 1.0 {
                gfx.multiply_alpha(alpha);
            }

            gfx.set_blend_mode(sprite.blend_mode);

            let matrix = sprite.local_matrix();

            let mut m02 = matrix.m02;
            let mut m12 = matrix.m12;
            if sprite.pixel_snapping {
                // Snap the translation to the nearest whole pixel
                m02 = m02.round();
                m12 = m12.round();
            }
            gfx.transform(matrix.m00, matrix.m10, matrix.m01, matrix.m11, m02, m12);

            let scissor = &sprite.scissor;
            if let Some(ref scissor) = scissor {
                gfx.apply_scissor(scissor.x, scissor.y, scissor.width, scissor.height);
            }

            sprite.draw(gfx);
        }

        // Render any partially occluded director scenes
        // FIXME:
        // let director = Director::global();

        // let scenes = &director.occluded_scenes;
        // for scene in scenes {
        //     Self::render(scene, gfx);
        // }

        // Render all children
        let mut child = entity.first_child;
        while let Some(item) = child {
            let entity = item.entity();
            let next = entity.next;
            Self::render(&entity, gfx);
            child = next;
        }

        // If save() was called, unwind it
        if sprite.is_some() {
            gfx.restore();
        }
    }

    /// The "natural" width of this sprite, without any transformations being applied. Used for hit
    /// testing. This does not consider child sprites, use Sprite.getBounds for that.
    pub fn natural_width(&self) -> f32 {
        0.0
    }

    /// The "natural" height of this sprite, without any transformations being applied. Used for hit
    /// testing. This does not consider child sprites, use Sprite.getBounds for that.
    pub fn natural_height(&self) -> f32 {
        0.0
    }

    /// Returns true if the given point (in viewport/stage coordinates) lies inside this sprite.
    pub fn contains(&mut self, view_x: f32, view_y: f32) -> bool {
        if let Some(pt) = self.view_matrix().inverse_transform(view_x, view_y) {
            return self.contains_local(pt.x, pt.y);
        }
        false
    }

    /// Returns true if the given point (in local coordinates) lies inside this sprite.
    pub fn contains_local(&self, local_x: f32, local_y: f32) -> bool {
        local_x >= 0.0
            && local_x < self.natural_width()
            && local_y >= 0.0
            && local_y < self.natural_height()
    }

    /// Returns the local transformation matrix, relative to the parent. This matrix may be modified
    /// to position the sprite, but any changes will be invalidated when the x, y, scaleX, scaleY,
    /// rotation, anchorX, or anchorY properties are updated.
    pub fn local_matrix(&mut self) -> Matrix {
        if self.component.flags.contains(Self::LOCAL_MATRIX_DIRTY) {
            self.component.flags = self.component.flags.remove(Self::LOCAL_MATRIX_DIRTY);

            if self.component.flags.contains(Self::ROTATION_DIRTY) {
                self.component.flags = self.component.flags.remove(Self::ROTATION_DIRTY);
                let rotation: f32 = Math::to_radians(self.rotation.get());
                self.sin_cache = rotation.sin();
                self.cos_cache = rotation.cos();
            }

            let scale_x: f32 = self.scale_x.get();
            let scale_y: f32 = self.scale_y.get();
            self.local_matrix.set(
                self.cos_cache * scale_x,
                self.sin_cache * scale_x,
                -self.sin_cache * scale_y,
                self.cos_cache * scale_y,
                self.x.get(),
                self.y.get(),
            );
            self.local_matrix
                .translate(-self.anchor_x.get(), -self.anchor_y.get());
        }

        self.local_matrix
    }

    /// Returns the view transformation matrix, relative to the root. Do NOT modify this matrix.
    pub fn view_matrix(&mut self) -> Matrix {
        if self.is_view_matrix_dirty() {
            let mut parent_sprite = self.parent_sprite();
            self.view_matrix = if let Some(ref mut parent) = parent_sprite {
                Matrix::multiply(parent.view_matrix(), self.local_matrix())
            } else {
                self.local_matrix()
            };

            self.component.flags = self.component.flags.remove(Self::VIEW_MATRIX_DIRTY);
            if let Some(ref parent) = parent_sprite {
                self.parent_view_matrix_update_count = parent.view_matrix_update_count;
            }
            self.view_matrix_update_count += 1;
        }

        self.view_matrix
    }

    /// Chainable convenience method to set the anchor position.
    /// @returns This instance, for chaining.
    pub fn set_anchor(&mut self, x: f32, y: f32) -> &Self {
        self.anchor_x.set(x);
        self.anchor_y.set(y);

        self
    }

    /// Chainable convenience method to center the anchor using the natural width and height.
    /// @returns This instance, for chaining.
    pub fn center_anchor(&mut self) -> &Self {
        self.anchor_x.set(self.natural_width() / 2.0);
        self.anchor_y.set(self.natural_height() / 2.0);

        self
    }

    /// Chainable convenience method to set the position.
    /// @returns This instance, for chaining.
    pub fn set_xy(&mut self, x: f32, y: f32) -> &Self {
        self.x.set(x);
        self.y.set(y);

        self
    }

    /// Chainable convenience method to set the alpha.
    /// @returns This instance, for chaining.
    pub fn set_alpha(&mut self, alpha: f32) -> &Self {
        self.alpha.set(alpha);

        self
    }

    /// Chainable convenience method to set the rotation.
    /// @returns This instance, for chaining.
    pub fn set_rotation(&mut self, rotation: f32) -> &Self {
        self.rotation.set(rotation);

        self
    }

    /// Chainable convenience method to uniformly set the scale.
    /// @returns This instance, for chaining.
    pub fn set_scale(&mut self, scale: f32) -> &Self {
        self.scale_x.set(scale);
        self.scale_y.set(scale);

        self
    }

    /// Chainable convenience method to set the scale.
    /// @returns This instance, for chaining.
    pub fn set_scale_xy(&mut self, scale_x: f32, scale_y: f32) -> &Self {
        self.scale_x.set(scale_x);
        self.scale_y.set(scale_y);

        self
    }

    /// Chainable convenience method to set the blendMode.
    /// @returns This instance, for chaining.
    pub fn set_blend_mode(&mut self, blend_mode: BlendMode) -> &Self {
        self.blend_mode = blend_mode;

        self
    }

    /// Chainable convenience method to set pointerEnabled to false.
    /// @returns This instance, for chaining.
    pub fn disable_pointer(&mut self) -> &Self {
        self.pointer_enabled = false;

        self
    }

    /// Chainable convenience method to set pixelSnapping to false.
    /// @returns This instance, for chaining.
    pub fn disable_pixel_snapping(&mut self) -> &Self {
        self.pixel_snapping = false;

        self
    }

    // override
    pub fn on_added(&self) {
        if self.component.flags.contains(Self::HOVERING) {
            self.connect_hover();
        }
    }

    // override
    pub fn on_removed(&mut self) {
        if let Some(ref mut conn) = self.hover_connection {
            conn.dispose();
            self.hover_connection = None;
        }
    }

    // override
    pub fn on_update(&mut self, dt: f32) {
        self.x.update(dt);
        self.y.update(dt);
        self.rotation.update(dt);
        self.scale_x.update(dt);
        self.scale_y.update(dt);
        self.alpha.update(dt);
        self.anchor_x.update(dt);
        self.anchor_y.update(dt);
    }

    /// Draws this sprite to the given Graphics.
    pub fn draw(&self, gfx: &Box<dyn Graphics>) {
        // See subclasses
    }

    pub fn is_view_matrix_dirty(&self) -> bool {
        if self.component.flags.contains(Self::VIEW_MATRIX_DIRTY) {
            return true;
        }

        if let Some(ref parent_sprite) = self.parent_sprite() {
            return self.parent_view_matrix_update_count != parent_sprite.view_matrix_update_count
                || parent_sprite.is_view_matrix_dirty();
        }

        false
    }

    pub fn parent_sprite(&self) -> Option<Sprite> {
        if let Some(ref owner) = self.component.owner {
            let mut parent = owner.entity().parent;
            while let Some(item) = parent {
                let entity = item.entity();
                let sprite = EntityManager::<Sprite>::get(&entity);
                if sprite.is_some() {
                    return sprite;
                }
                parent = entity.parent;
            }
        }

        None
    }

    pub fn pointer_down(&mut self) -> &Option<Signal1<PointerEvent>> {
        if self.pointer_down.is_none() {
            self.pointer_down = Some(Signal1::new(None));
        }

        &self.pointer_down
    }

    pub fn pointer_move(&mut self) -> &Option<Signal1<PointerEvent>> {
        if self.pointer_move.is_none() {
            self.pointer_move = Some(Signal1::new(None));
        }

        &self.pointer_move
    }

    pub fn pointer_up(&mut self) -> &Option<Signal1<PointerEvent>> {
        if self.pointer_up.is_none() {
            self.pointer_up = Some(Signal1::new(None));
        }

        &self.pointer_up
    }

    pub fn pointer_in(&mut self) -> &Option<Signal1<PointerEvent>> {
        if self.pointer_in.is_none() {
            self.pointer_in = Some(Signal1::new(None));
        }

        &self.pointer_in
    }

    pub fn pointer_out(&mut self) -> &Option<Signal1<PointerEvent>> {
        if self.pointer_out.is_none() {
            self.pointer_out = Some(Signal1::new(None));
        }

        &self.pointer_out
    }

    pub fn connect_hover(&self) {
        if self.hover_connection.is_some() {
            return;
        }

        // let owner = self.inner.owner;
        // self.hover_connection = Some(System::pointer().get_move().connect(
        //     Box::new(move |event: &PointerEvent| {
        //         // Return early if this sprite was in the event chain
        //         let hit = event.hit;
        //         while let Some(ref hitsprite) = hit {
        //             if hitsprite.inner.owner == owner {
        //                 return;
        //             }
        //             hit = hitsprite.get_parent_sprite();
        //         }

        //         // This sprite is not under the pointer
        //         if let Some(ref pointer_out) = self.pointer_out {
        //             if self.inner.flags.contains(Self::HOVERING) {
        //                 pointer_out.emit(event.clone());
        //             }
        //         }

        //         self.inner.flags = self.inner.flags.remove(Self::HOVERING);

        //         if let Some(ref hover_connection) = self.hover_connection {
        //             hover_connection.dispose();
        //             self.hover_connection = None;
        //         }
        //     }),
        //     false,
        // ));
    }

    #[inline]
    pub fn visible(&self) -> bool {
        self.component.flags.contains(Self::VISIBLE)
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.component.flags = self.component.flags.set(Self::VISIBLE, visible);
    }

    #[inline]
    pub fn pointer_enabled(&self) -> bool {
        self.component.flags.contains(Self::POINTER_ENABLED)
    }

    pub fn set_pointer_enabled(&mut self, pointer_enabled: bool) {
        self.component.flags = self
            .component
            .flags
            .set(Self::POINTER_ENABLED, pointer_enabled);
    }

    #[inline]
    pub fn pixel_snapping(&self) -> bool {
        self.component.flags.contains(Self::PIXEL_SNAPPING)
    }

    pub fn set_pixel_snapping(&mut self, pixel_snapping: bool) {
        self.component.flags = self
            .component
            .flags
            .set(Self::PIXEL_SNAPPING, pixel_snapping);
    }

    pub fn on_pointer_down(&mut self, event: PointerEvent) {
        self.on_hover(event.clone());
        if let Some(ref pointer_down) = self.pointer_down {
            pointer_down.emit(event);
        }
    }

    pub fn on_pointer_move(&mut self, event: PointerEvent) {
        self.on_hover(event.clone());
        if let Some(ref pointer_move) = self.pointer_move {
            pointer_move.emit(event);
        }
    }

    pub fn on_hover(&mut self, event: PointerEvent) {
        if self.component.flags.contains(Self::HOVERING) {
            return; // Already hovering
        }
        self.component.flags = self.component.flags.add(Self::HOVERING);

        // Notify listeners and connect the hover-out listener
        if self.pointer_in.is_some() || self.pointer_out.is_some() {
            //  TODO: should deal with logic
            if let Some(ref pointer_in) = self.pointer_in {
                pointer_in.emit(event);
            }
            self.connect_hover();
        }
    }

    pub fn on_pointer_up(&mut self, event: PointerEvent) {
        match &event.source {
            Some(EventSource::Touch { point }) => {
                if let Some(ref pointer_out) = self.pointer_out {
                    if self.component.flags.contains(Self::HOVERING) {
                        pointer_out.emit(event.clone());
                    }
                }

                self.component.flags = self.component.flags.remove(Self::HOVERING);
                if let Some(ref mut hover_connection) = self.hover_connection {
                    hover_connection.dispose();
                    self.hover_connection = None;
                }
            }
            _ => {}
        }

        if let Some(ref pointer_up) = self.pointer_up {
            pointer_up.emit(event.clone());
        }
    }

    // static
    pub fn hit_test_backwards(entity: Option<Entity>, x: f32, y: f32) -> Option<Sprite> {
        if let Some(ref entity) = entity {
            if let Some(next) = entity.next {
                let result = Self::hit_test_backwards(Some(next.entity()), x, y);
                if result.is_some() {
                    return result;
                } else {
                    return Self::hit_test(entity.clone(), x, y);
                }
            }
        }

        None
    }

    // static
    fn bounds_impl(entity: &Entity, matrix: Option<Matrix>, result: &mut Rectangle) {
        if let Some(ref mut sprite) = EntityManager::<Sprite>::get(entity) {
            let matrix = if let Some(matrix) = matrix {
                Matrix::multiply(matrix, sprite.local_matrix()) // Allocation!
            } else {
                sprite.local_matrix()
            };

            let x1 = 0.0;
            let y1 = 0.0;
            let x2 = sprite.natural_width();
            let y2 = sprite.natural_height();

            // Intersecting scissor rectangles are too tricky for bounds calculation, ignore it for
            // now..
            // let scissor = sprite.scissor;
            // if scissor.is_some() {
            //     x1 = x1.max(scissor.x);
            //     y1 = y1.max(scissor.y);
            //     x2 = x2.min(scissor.x + scissor.width);
            //     y2 = y2.min(scissor.y + scissor.height);
            // }

            // Extend the rectangle out to fit this sprite
            if x2 > x1 && y2 > y1 {
                Self::extend_rect(matrix, x1, y1, result);
                Self::extend_rect(matrix, x2, y1, result);
                Self::extend_rect(matrix, x2, y2, result);
                Self::extend_rect(matrix, x1, y2, result);
            }
        }

        // Recurse into partially occluded director scenes
        // let director = Director::global();

        // let scenes = &director.occluded_scenes;
        // let mut idx = 0;
        // let ll = scenes.len();
        // while idx < ll {
        //     Self::get_bounds_impl(&scenes[idx], matrix, result);
        //     idx += 1;
        // }

        // Recurse into all children
        let mut child = entity.first_child;
        while let Some(item) = child {
            let entity = item.entity();
            let next = entity.next;
            Self::bounds_impl(&entity, matrix, result);
            child = next;
        }
    }

    // static
    #[allow(unused_mut)]
    pub fn extend_rect(matrix: Matrix, x: f32, y: f32, mut rect: &mut Rectangle) {
        // let p = matrix.transform(x, y, self.scratch_point);
        // x = p.x;
        // y = p.y;

        // // The width and height of the rectangle are treated like the bottom right point, rather
        // // than a true width and height offset
        // if x < rect.x {
        //     rect.x = x;
        // }

        // if y < rect.y {
        //     rect.y = y;
        // }

        // if x > rect.width {
        //     rect.width = x;
        // }

        // if y > rect.height {
        //     rect.height = y;
        // }
        unimplemented!()
    }
}

impl AsRef<Sprite> for Sprite {
    fn as_ref(&self) -> &Sprite {
        &self
    }
}

impl AsRef<Component> for Sprite {
    fn as_ref(&self) -> &Component {
        &self.component
    }
}

impl fmt::Debug for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sprite")
            //  .field("x", &self.x)
            //  .field("y", &self.y)
            .finish()
    }
}
