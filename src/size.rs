use std::f32;

/// An amount of space in 2 dimensions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size<T = f32> {
    /// The width.
    pub width: T,
    /// The height.
    pub height: T,
}

impl<T> Size<T> {
    /// Creates a new  [`Size`] with the given width and height.
    ///
    /// [`Size`]: struct.Size.html
    pub const fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

impl Size {
    /// A [`Size`] with zero width and height.
    ///
    /// [`Size`]: struct.Size.html
    pub const ZERO: Self = Self::new(0., 0.);

    /// A [`Size`] with a width and height of 1 unit.
    ///
    /// [`Size`]: struct.Size.html
    pub const UNIT: Self = Self::new(1., 1.);

    /// A [`Size`] with infinite width and height.
    ///
    /// [`Size`]: struct.Size.html
    pub const INFINITY: Self = Self::new(f32::INFINITY, f32::INFINITY);

    /// Increments the [`Size`] to account for the given padding.
    ///
    /// [`Size`]: struct.Size.html
    pub fn pad(&self, padding: f32) -> Self {
        Self {
            width: self.width + padding * 2.0,
            height: self.height + padding * 2.0,
        }
    }
}

impl From<[f32; 2]> for Size {
    fn from([width, height]: [f32; 2]) -> Self {
        Self { width, height }
    }
}

impl From<[u16; 2]> for Size {
    fn from([width, height]: [u16; 2]) -> Self {
        Self::new(width.into(), height.into())
    }
}
