// use std::fmt;

// /// A 2D point or vector.
// #[derive(Default, Clone, Debug)]
// pub struct Point {
//     pub x: f32,
//     pub y: f32,
// }

// impl Point {
//     pub fn new(x: f32, y: f32) -> Self {
//         Self { x, y }
//     }

//     /// Re-assign the coordinates of this point.
//     pub fn set(&mut self, x: f32, y: f32) {
//         self.x = x;
//         self.y = y;
//     }

//     /// Normalize this vector, so that its new magnitude is 1.
//     pub fn normalize(&mut self) {
//         let m = self.magnitude();
//         self.x /= m;
//         self.y /= m;
//     }

//     /// The dot product of two vectors.
//     pub fn dot(&self, x: f32, y: f32) -> f32 {
//         self.x * x + self.y * y
//     }

//     /// Scales a vector's magnitude by a given value.
//     pub fn multiply(&mut self, n: f32) {
//         self.x *= n;
//         self.y *= n;
//     }

//     /// The magnitude (length) of this vector.
//     pub fn magnitude(&self) -> f32 {
//         (self.x * self.x + self.y * self.y).sqrt()
//     }

//     /// The distance between this point and another point.
//     pub fn distance_to(&self, x: f32, y: f32) -> f32 {
//         self.distance_to_squared(x, y).sqrt()
//     }

//     /// The squared distance between this point and another point. This is a faster operation than
//     /// distanceTo.
//     pub fn distance_to_squared(&self, x: f32, y: f32) -> f32 {
//         let dx = self.x - x;
//         let dy = self.y - y;

//         dx * dx + dy * dy
//     }

//     pub fn equals(&self, other: Point) -> bool {
//         self.x == other.x && self.y == other.y
//     }
//     // static
//     pub fn pointMake(x: f32, y: f32) -> Point2<f32> {
//         Point2::new(x, y)
//     }

//     // static
//     pub fn p(x: f32, y: f32) -> Point2<f32> {
//         Point2::new(x, y)
//     }

//     // static
//     pub fn pointZero() -> Point2<f32> {
//         Point2::new(0, 0)
//     }

//     // static
//     pub fn pointEqualToPoint(p1: Point2<f32>, p2: Point2<f32>) -> bool {
//         if p1 == None || p2 == None {
//             return false;
//         }

//         (p1.x == p2.x) && (p1.y == p2.y)
//     }
// }

// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "({},{})", self.x, self.y)
//     }
// }
