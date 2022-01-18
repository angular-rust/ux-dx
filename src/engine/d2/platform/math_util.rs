pub struct MathUtil {}

impl MathUtil {
    /// Returns the smallest power of two >= n.
    //  static
    pub fn next_power_of_two(n: i32) -> i32 {
        let mut p = 1;
        while p < n {
            p <<= 1;
        }
        return p;
    }
}
