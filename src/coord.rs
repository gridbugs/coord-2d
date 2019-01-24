#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Axis {
    X,
    Y,
}

impl Axis {
    pub fn other(self) -> Self {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::X,
        }
    }
    pub fn new_coord(self, this_axis: i32, other_axis: i32) -> Coord {
        match self {
            Axis::X => Coord::new(this_axis, other_axis),
            Axis::Y => Coord::new(other_axis, this_axis),
        }
    }
    pub fn try_new_size(
        self,
        this_axis: u32,
        other_axis: u32,
    ) -> Result<Size, DimensionTooLargeForSize> {
        match self {
            Axis::X => Size::try_new(this_axis, other_axis),
            Axis::Y => Size::try_new(other_axis, this_axis),
        }
    }
    pub fn new_size(self, this_axis: u32, other_axis: u32) -> Size {
        match self {
            Axis::X => Size::new(this_axis, other_axis),
            Axis::Y => Size::new(other_axis, this_axis),
        }
    }
}

pub trait StaticAxis: private::Sealed {
    type Other;
    fn axis() -> Axis;
    fn new_coord(this_axis: i32, other_axis: i32) -> Coord;
    fn coord_get(coord: Coord) -> i32;
    fn coord_get_mut(coord: &mut Coord) -> &mut i32;
    fn coord_with_axis<F: FnMut(i32) -> i32>(coord: Coord, f: F) -> Coord;
    fn coord_set(coord: Coord, value: i32) -> Coord;
    fn coord_set_in_place(coord: &mut Coord, value: i32);
    fn try_new_size(this_axis: u32, other_axis: u32) -> Result<Size, DimensionTooLargeForSize>;
    fn size_get(size: Size) -> u32;
    fn size_get_mut(size: &mut Size) -> &mut u32;
    fn size_with_axis<F: FnMut(u32) -> u32>(size: Size, f: F) -> Size;
    fn try_size_set(size: Size, value: u32) -> Result<Size, DimensionTooLargeForSize>;
    fn try_size_set_in_place(size: &mut Size, value: u32) -> Result<(), DimensionTooLargeForSize>;
    fn size_set(size: Size, value: u32) -> Size {
        match Self::try_size_set(size, value) {
            Err(DimensionTooLargeForSize) => {
                panic!("Value is too big: {}. Max is {}.", value, SIZE_MAX);
            }
            Ok(size) => size,
        }
    }
    fn size_set_in_place(size: &mut Size, value: u32) {
        match Self::try_size_set_in_place(size, value) {
            Err(DimensionTooLargeForSize) => {
                panic!("Value is too big: {}. Max is {}.", value, SIZE_MAX);
            }
            Ok(()) => (),
        }
    }
    fn new_size(this_axis: u32, other_axis: u32) -> Size {
        match Self::try_new_size(this_axis, other_axis) {
            Err(DimensionTooLargeForSize) => {
                panic!(
                    "Size is too big: ({}, {}). Max is {}.",
                    this_axis, other_axis, SIZE_MAX
                );
            }
            Ok(size) => size,
        }
    }
}

pub mod static_axis {
    pub struct X;
    pub struct Y;
}

fn check_size_limit(value: u32) -> Result<(), DimensionTooLargeForSize> {
    if value > SIZE_MAX {
        Err(DimensionTooLargeForSize)
    } else {
        Ok(())
    }
}

impl StaticAxis for static_axis::X {
    type Other = static_axis::Y;
    fn axis() -> Axis {
        Axis::X
    }
    fn new_coord(this_axis: i32, other_axis: i32) -> Coord {
        Coord::new(this_axis, other_axis)
    }
    fn coord_get(coord: Coord) -> i32 {
        coord.x
    }
    fn coord_get_mut(coord: &mut Coord) -> &mut i32 {
        &mut coord.x
    }
    fn coord_with_axis<F: FnMut(i32) -> i32>(coord: Coord, mut f: F) -> Coord {
        Coord {
            x: f(coord.x),
            ..coord
        }
    }
    fn coord_set(coord: Coord, value: i32) -> Coord {
        Coord { x: value, ..coord }
    }
    fn coord_set_in_place(coord: &mut Coord, value: i32) {
        coord.x = value
    }
    fn try_new_size(this_axis: u32, other_axis: u32) -> Result<Size, DimensionTooLargeForSize> {
        Size::try_new(this_axis, other_axis)
    }
    fn size_get(size: Size) -> u32 {
        size.x
    }
    fn size_get_mut(size: &mut Size) -> &mut u32 {
        &mut size.x
    }
    fn size_with_axis<F: FnMut(u32) -> u32>(size: Size, mut f: F) -> Size {
        Size {
            x: f(size.x),
            ..size
        }
    }
    fn try_size_set(size: Size, value: u32) -> Result<Size, DimensionTooLargeForSize> {
        check_size_limit(value)?;
        Ok(Size { x: value, ..size })
    }
    fn try_size_set_in_place(size: &mut Size, value: u32) -> Result<(), DimensionTooLargeForSize> {
        check_size_limit(value)?;
        size.x = value;
        Ok(())
    }
}

impl StaticAxis for static_axis::Y {
    type Other = static_axis::X;
    fn axis() -> Axis {
        Axis::Y
    }
    fn new_coord(this_axis: i32, other_axis: i32) -> Coord {
        Coord::new(other_axis, this_axis)
    }
    fn coord_get(coord: Coord) -> i32 {
        coord.y
    }
    fn coord_get_mut(coord: &mut Coord) -> &mut i32 {
        &mut coord.y
    }
    fn coord_with_axis<F: FnMut(i32) -> i32>(coord: Coord, mut f: F) -> Coord {
        Coord {
            y: f(coord.y),
            ..coord
        }
    }
    fn coord_set(coord: Coord, value: i32) -> Coord {
        Coord { y: value, ..coord }
    }
    fn coord_set_in_place(coord: &mut Coord, value: i32) {
        coord.y = value
    }
    fn size_get(size: Size) -> u32 {
        size.y
    }
    fn size_get_mut(size: &mut Size) -> &mut u32 {
        &mut size.y
    }
    fn size_with_axis<F: FnMut(u32) -> u32>(size: Size, mut f: F) -> Size {
        Size {
            y: f(size.y),
            ..size
        }
    }
    fn try_size_set(size: Size, value: u32) -> Result<Size, DimensionTooLargeForSize> {
        check_size_limit(value)?;
        Ok(Size { y: value, ..size })
    }
    fn try_size_set_in_place(size: &mut Size, value: u32) -> Result<(), DimensionTooLargeForSize> {
        check_size_limit(value)?;
        size.y = value;
        Ok(())
    }
    fn try_new_size(this_axis: u32, other_axis: u32) -> Result<Size, DimensionTooLargeForSize> {
        Size::try_new(other_axis, this_axis)
    }
}

mod private {
    pub trait Sealed {}

    impl Sealed for super::static_axis::X {}
    impl Sealed for super::static_axis::Y {}
}

/// General purpose coordinate
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct NegativeDimension;

#[derive(Debug)]
pub struct DimensionTooLargeForSize;

#[derive(Debug)]
pub struct DimensionTooLargeForCoord;

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn from_size(size: Size) -> Result<Self, DimensionTooLargeForCoord> {
        size.to_coord()
    }
    pub fn to_size(self) -> Result<Size, NegativeDimension> {
        if self.x < 0 || self.y < 0 {
            Err(NegativeDimension)
        } else {
            Ok(Size::new(self.x as u32, self.y as u32))
        }
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
    pub fn get(self, axis: Axis) -> i32 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
        }
    }
    pub fn get_mut(&mut self, axis: Axis) -> &mut i32 {
        match axis {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
        }
    }
    pub fn with_axis<F: FnMut(i32) -> i32>(self, axis: Axis, mut f: F) -> Self {
        match axis {
            Axis::X => Self {
                x: f(self.x),
                ..self
            },
            Axis::Y => Self {
                y: f(self.y),
                ..self
            },
        }
    }
    pub fn set(self, axis: Axis, value: i32) -> Self {
        match axis {
            Axis::X => Self::new(value, self.y),
            Axis::Y => Self::new(self.x, value),
        }
    }
    pub fn set_in_place(&mut self, axis: Axis, value: i32) {
        match axis {
            Axis::X => self.x = value,
            Axis::Y => self.y = value,
        }
    }
    pub fn new_axis(this_axis: i32, other_axis: i32, axis: Axis) -> Self {
        axis.new_coord(this_axis, other_axis)
    }
    pub fn get_static<A: StaticAxis>(self) -> i32 {
        A::coord_get(self)
    }
    pub fn get_static_mut<A: StaticAxis>(&mut self) -> &mut i32 {
        A::coord_get_mut(self)
    }
    pub fn with_static_axis<A: StaticAxis, F: FnMut(i32) -> i32>(self, f: F) -> Self {
        A::coord_with_axis(self, f)
    }
    pub fn set_static<A: StaticAxis>(self, value: i32) -> Self {
        A::coord_set(self, value)
    }
    pub fn set_static_in_place<A: StaticAxis>(&mut self, value: i32) {
        A::coord_set_in_place(self, value)
    }
    pub fn new_static_axis<A: StaticAxis>(this_axis: i32, other_axis: i32) -> Self {
        A::new_coord(this_axis, other_axis)
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
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct Size {
    x: u32,
    y: u32,
}

const SIZE_MAX: u32 = ::std::i32::MAX as u32 + 1;

impl Size {
    pub fn try_new(width: u32, height: u32) -> Result<Self, DimensionTooLargeForSize> {
        check_size_limit(width)?;
        check_size_limit(height)?;
        Ok(Self {
            x: width,
            y: height,
        })
    }

    /// Creates a new `Size`.
    /// Panics if `width` or `width` is greater than `::std::i32::MAX as u32 + 1`.
    pub fn new(width: u32, height: u32) -> Self {
        match Self::try_new(width, height) {
            Err(DimensionTooLargeForSize) => {
                panic!(
                    "Size is too big: ({}, {}). Max is {}.",
                    width, width, SIZE_MAX
                );
            }
            Ok(size) => size,
        }
    }

    pub fn from_coord(coord: Coord) -> Result<Self, NegativeDimension> {
        coord.to_size()
    }

    pub fn to_coord(self) -> Result<Coord, DimensionTooLargeForCoord> {
        if self.x > ::std::i32::MAX as u32 || self.y > ::std::i32::MAX as u32 {
            Err(DimensionTooLargeForCoord)
        } else {
            Ok(Coord::new(self.x as i32, self.y as i32))
        }
    }

    pub fn get(self, axis: Axis) -> u32 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
        }
    }

    pub fn get_mut(&mut self, axis: Axis) -> &mut u32 {
        match axis {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
        }
    }

    pub fn with_axis<F: FnMut(u32) -> u32>(self, axis: Axis, mut f: F) -> Self {
        match axis {
            Axis::X => Self {
                x: f(self.x),
                ..self
            },
            Axis::Y => Self {
                y: f(self.y),
                ..self
            },
        }
    }

    pub fn try_set(self, axis: Axis, value: u32) -> Result<Self, DimensionTooLargeForSize> {
        check_size_limit(value)?;
        Ok(match axis {
            Axis::X => Self {
                x: value,
                y: self.y,
            },
            Axis::Y => Self {
                x: self.x,
                y: value,
            },
        })
    }

    pub fn set(self, axis: Axis, value: u32) -> Self {
        match self.try_set(axis, value) {
            Err(DimensionTooLargeForSize) => {
                panic!("Value is too big: {}. Max is {}.", value, SIZE_MAX);
            }
            Ok(size) => size,
        }
    }

    pub fn try_set_in_place(
        &mut self,
        axis: Axis,
        value: u32,
    ) -> Result<(), DimensionTooLargeForSize> {
        check_size_limit(value)?;
        match axis {
            Axis::X => self.x = value,
            Axis::Y => self.y = value,
        }
        Ok(())
    }

    pub fn set_in_place(&mut self, axis: Axis, value: u32) {
        match self.try_set_in_place(axis, value) {
            Err(DimensionTooLargeForSize) => {
                panic!("Value is too big: {}. Max is {}.", value, SIZE_MAX);
            }
            Ok(()) => (),
        }
    }

    pub fn try_new_axis(
        this_axis: u32,
        other_axis: u32,
        axis: Axis,
    ) -> Result<Self, DimensionTooLargeForSize> {
        axis.try_new_size(this_axis, other_axis)
    }

    pub fn new_axis(this_axis: u32, other_axis: u32, axis: Axis) -> Self {
        axis.new_size(this_axis, other_axis)
    }

    pub fn get_static<A: StaticAxis>(self) -> u32 {
        A::size_get(self)
    }
    pub fn get_static_mut<A: StaticAxis>(&mut self) -> &mut u32 {
        A::size_get_mut(self)
    }
    pub fn with_static_axis<A: StaticAxis, F: FnMut(u32) -> u32>(self, f: F) -> Self {
        A::size_with_axis(self, f)
    }
    pub fn try_set_static<A: StaticAxis>(
        self,
        value: u32,
    ) -> Result<Self, DimensionTooLargeForSize> {
        A::try_size_set(self, value)
    }
    pub fn try_set_static_in_place<A: StaticAxis>(
        &mut self,
        value: u32,
    ) -> Result<(), DimensionTooLargeForSize> {
        A::try_size_set_in_place(self, value)
    }

    pub fn set_static<A: StaticAxis>(self, value: u32) -> Self {
        A::size_set(self, value)
    }

    pub fn set_static_in_place<A: StaticAxis>(&mut self, value: u32) {
        A::size_set_in_place(self, value)
    }

    pub fn try_new_static_axis<A: StaticAxis>(
        this_axis: u32,
        other_axis: u32,
    ) -> Result<Self, DimensionTooLargeForSize> {
        A::try_new_size(this_axis, other_axis)
    }

    pub fn new_static_axis<A: StaticAxis>(this_axis: u32, other_axis: u32) -> Self {
        A::new_size(this_axis, other_axis)
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
