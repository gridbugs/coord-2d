/// General purpose coordinate
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn normalize_part(value: i32, size: u32) -> i32 {
        let value = value % size as i32;
        if value < 0 {
            value + size as i32
        } else {
            value
        }
    }
    pub fn normalize(self, size: Size) -> Self {
        Self {
            x: Self::normalize_part(self.x, size.x()),
            y: Self::normalize_part(self.y, size.y()),
        }
    }
    pub fn is_valid(self, size: Size) -> bool {
        if self.x < 0 || self.y < 0 {
            return false;
        }

        let x = self.x as u32;
        let y = self.y as u32;

        x < size.x() && y < size.y()
    }
}

impl From<(i32, i32)> for Coord {
    fn from((x, y): (i32, i32)) -> Self {
        Coord::new(x, y)
    }
}

impl From<[i32; 2]> for Coord {
    fn from(array: [i32; 2]) -> Self {
        Coord::new(array[0], array[1])
    }
}

/// A size cannot be created which would contain un-addressable cells.
/// That is, the maximum size has a width and height of one greater than the maximum `i32`.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct Size {
    x: u32,
    y: u32,
}

impl Size {
    /// Creates a new `Size`.
    /// Panics if `x` or `y` is greater than `::std::i32::MAX as u32 + 1`.
    pub fn new(x: u32, y: u32) -> Self {
        const SIZE_MAX: u32 = ::std::i32::MAX as u32 + 1;
        if x > SIZE_MAX || y > SIZE_MAX {
            panic!("Size is too big: ({}, {})", x, y);
        }
        Self { x, y }
    }

    /// Returns the width.
    pub fn width(self) -> u32 {
        self.x
    }

    /// Alias for `width`.
    pub fn x(self) -> u32 {
        self.x
    }

    /// Returns the height.
    pub fn height(self) -> u32 {
        self.y
    }

    /// Alias for `height`.
    pub fn y(self) -> u32 {
        self.y
    }

    /// Return the number of cells in a 2D grid of this size.
    pub fn count(self) -> usize {
        (self.x * self.y) as usize
    }
}

impl From<(u32, u32)> for Size {
    fn from((x, y): (u32, u32)) -> Self {
        Size::new(x, y)
    }
}

impl From<[u32; 2]> for Size {
    fn from(array: [u32; 2]) -> Self {
        Size::new(array[0], array[1])
    }
}

#[cfg(test)]
mod test {
    use super::{Coord, Size};

    #[test]
    fn normalize() {
        assert_eq!(
            Coord::new(5, 2).normalize(Size::new(2, 3)),
            Coord::new(1, 2)
        );
        assert_eq!(
            Coord::new(-4, 3).normalize(Size::new(3, 1)),
            Coord::new(2, 0)
        );
    }
}
