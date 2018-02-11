#[derive(Clone, Copy, Default)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl Size {
    /// Returns a new `Size` of the given dimensions
    pub fn new(width: i32, height: i32) -> Size {
        Size {
            width: width,
            height: height,
        }
    }
}
