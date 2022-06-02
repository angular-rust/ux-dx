use std::fmt;

/// A 2D rectangle.
#[derive(Default, Clone, Debug)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    // /// The X-coordinate of the left side of the rectangle.
    // pub left (get, None) -> f32;

    // /// The X-coordinate of the right side of the rectangle.
    // pub right (get, None) -> f32;

    // /// The Y-coordinate of the top side of the rectangle.
    // pub top (get, None) -> f32;

    // /// The Y-coordinate of the bottom side of the rectangle.
    // pub bottom (get, None) -> f32;

    // /// The X-coordinate of the center of the rectangle.
    // pub centerX (get, None) -> f32;

    // /// The Y-coordinate of the center of the rectangle.
    // pub centerY (get, None) -> f32;
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn set(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;
    }

    /// Returns true if this rectangle contains the given point.
    pub fn contains(&self, x: f32, y: f32) -> bool {
        // A little more complicated than usual due to proper handling of negative widths/heights

        let x = x - self.x;
        if self.width >= 0.0 {
            if x < 0.0 || x > self.width {
                return false;
            }
        } else if x > 0.0 || x < self.width {
            return false;
        }

        let y = y - self.y;
        if self.height >= 0.0 {
            if y < 0.0 || y > self.height {
                return false;
            }
        } else if y > 0.0 || y < self.height {
            return false;
        }

        true
    }

    /// Returns whether this rectangle intersects another rectangle.
    ///  *
    /// @param rect The other rectangle to check for intersection.
    /// @param result If supplied and the rectangles intersect, will be set to the calculated
    ///   intersection rectangle.
    pub fn intersects(&self, rect: &Rectangle) -> Option<Rectangle> {
        let left = self.left().max(rect.left());
        let right = self.right().min(rect.right());
        if left > right {
            return None;
        }

        let top = self.top().max(rect.top());
        let bottom = self.bottom().min(rect.bottom());
        if top > bottom {
            return None;
        }

        Some(Rectangle {
            x: left,
            y: top,
            width: right - left,
            height: bottom - top,
        })
    }

    #[inline]
    fn left(&self) -> f32 {
        self.x
    }

    #[inline]
    fn top(&self) -> f32 {
        self.y
    }

    fn right(&self) -> f32 {
        self.x + self.width
    }

    fn bottom(&self) -> f32 {
        self.y + self.height
    }

    fn center_x(&self) -> f32 {
        self.x + self.width / 2.0
    }

    fn center_y(&self) -> f32 {
        self.y + self.height / 2.0
    }

    // static
    pub fn make(x: f32, y: f32, width: f32, height: f32) -> Rectangle {
        Rectangle::new(x, y, width, height)
    }

    // static
    pub fn zero() -> Rectangle {
        Rectangle::new(0.0, 0.0, 0.0, 0.0)
    }

    // static
    pub fn rect_equal_to_rect(rect1: &Rectangle, rect2: &Rectangle) -> bool {
        (rect1.x == rect2.x)
            && (rect1.y == rect2.y)
            && (rect1.width == rect2.width)
            && (rect1.height == rect2.height)
    }

    // static
    pub fn max_x(rect: &Rectangle) -> f32 {
        rect.x + rect.width
    }

    // static
    pub fn mid_x(rect: &Rectangle) -> f32 {
        (rect.x + rect.width) / 2.0
    }

    // static
    pub fn min_x(rect: &Rectangle) -> f32 {
        rect.x
    }

    // static
    pub fn max_y(rect: &Rectangle) -> f32 {
        rect.y + rect.height
    }

    // static
    pub fn mid_y(rect: &Rectangle) -> f32 {
        (rect.y + rect.height) / 2.0
    }

    // static
    pub fn min_y(rect: &Rectangle) -> f32 {
        rect.y
    }

    // static
    pub fn _rect_equal_to_zero(rect: &Rectangle) -> bool {
        rect.x == 0.0 && rect.y == 0.0 && rect.width == 0.0 && rect.height == 0.0
    }

    /// Returns the smallest rectangle that contains the two source rectangles.
    /// @function
    /// @param {cc::Rect} rectA
    /// @param {cc::Rect} rectB
    /// @return {cc::Rect}
    // static
    pub fn rect_union(rect_a: &Rectangle, rect_b: &Rectangle) -> Rectangle {
        let mut rect: Rectangle = Rectangle::new(0.0, 0.0, 0.0, 0.0);
        rect.x = rect_a.x.min(rect_b.x);
        rect.y = rect_a.y.min(rect_b.y);
        rect.width = (rect_a.x + rect_a.width).max(rect_b.x + rect_b.width) - rect.x;
        rect.height = (rect_a.y + rect_a.height).max(rect_b.y + rect_b.height) - rect.y;

        rect
    }
    // static
    pub fn rect_intersects_rect(rect_a: &Rectangle, rect_b: &Rectangle) -> bool {
        !(Self::max_x(rect_a) < Self::min_x(rect_b)
            || Self::max_x(rect_b) < Self::min_x(rect_a)
            || Self::max_y(rect_a) < Self::min_y(rect_b)
            || Self::max_y(rect_b) < Self::min_y(rect_a))
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{} {}x{})", self.x, self.y, self.width, self.height)
    }
}

impl PartialEq for Rectangle {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.y == other.y
            && self.width == other.width
            && self.height == other.height
    }
}
