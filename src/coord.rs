#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

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
    type Other: StaticAxis;
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
                panic!("Value is too big: {}. Max is {}.", value, MAX_SIZE_FIELD);
            }
            Ok(size) => size,
        }
    }
    fn size_set_in_place(size: &mut Size, value: u32) {
        match Self::try_size_set_in_place(size, value) {
            Err(DimensionTooLargeForSize) => {
                panic!("Value is too big: {}. Max is {}.", value, MAX_SIZE_FIELD);
            }
            Ok(()) => (),
        }
    }
    fn new_size(this_axis: u32, other_axis: u32) -> Size {
        match Self::try_new_size(this_axis, other_axis) {
            Err(DimensionTooLargeForSize) => {
                panic!(
                    "Size is too big: ({}, {}). Max is {}.",
                    this_axis, other_axis, MAX_SIZE_FIELD
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
    if value > MAX_SIZE_FIELD {
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
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn from_size(size: Size) -> Result<Self, DimensionTooLargeForCoord> {
        size.to_coord()
    }
    #[cfg(feature = "rand")]
    pub fn random_within<R: rand::Rng>(size: Size, rng: &mut R) -> Self {
        let x = rng.gen_range(0..size.width() as i32);
        let y = rng.gen_range(0..size.height() as i32);
        Self { x, y }
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
    pub fn constrain(mut self, size: Size) -> Option<Self> {
        if self.x < 0 {
            self.x = 0;
        }
        if self.y < 0 {
            self.y = 0;
        }
        let max_x = size.width().checked_sub(1)? as i32;
        if self.x > max_x {
            self.x = max_x;
        }
        let max_y = size.height().checked_sub(1)? as i32;
        if self.y > max_y {
            self.y = max_y;
        }
        Some(self)
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
    pub fn set_x(self, x: i32) -> Self {
        Self { x, ..self }
    }
    pub fn set_y(self, y: i32) -> Self {
        Self { y, ..self }
    }
    pub fn set_x_in_place(&mut self, x: i32) {
        self.x = x;
    }
    pub fn set_y_in_place(&mut self, y: i32) {
        self.y = y;
    }
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.x
            .checked_add(rhs.x)
            .and_then(|x| self.y.checked_add(rhs.y).map(|y| Self::new(x, y)))
    }
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.x
            .checked_sub(rhs.x)
            .and_then(|x| self.y.checked_sub(rhs.y).map(|y| Self::new(x, y)))
    }
    pub fn checked_mul(self, rhs: i32) -> Option<Self> {
        self.x
            .checked_mul(rhs)
            .and_then(|x| self.y.checked_mul(rhs).map(|y| Self::new(x, y)))
    }
    pub fn checked_div(self, rhs: i32) -> Option<Self> {
        self.x
            .checked_div(rhs)
            .and_then(|x| self.y.checked_div(rhs).map(|y| Self::new(x, y)))
    }
    pub const fn magnitude2(self) -> u32 {
        (self.x * self.x) as u32 + (self.y * self.y) as u32
    }
    pub const fn distance2(self, other: Self) -> u32 {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
        .magnitude2()
    }
    pub const fn manhattan_magnitude(self) -> u32 {
        self.x.abs() as u32 + self.y.abs() as u32
    }
    pub const fn manhattan_distance(self, other: Self) -> u32 {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
        .manhattan_magnitude()
    }
    pub const fn opposite(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
    pub const fn left90(self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }
    pub const fn right90(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
    pub const fn cardinal_left45(self) -> Self {
        Self {
            x: self.y + self.x,
            y: self.y - self.x,
        }
    }
    pub const fn cardinal_right45(self) -> Self {
        Self {
            x: self.x - self.y,
            y: self.y + self.x,
        }
    }
    pub const fn cardinal_left135(self) -> Self {
        Self {
            x: self.y - self.x,
            y: -self.x - self.y,
        }
    }
    pub const fn cardinal_right135(self) -> Self {
        Self {
            x: -self.y - self.x,
            y: self.x - self.y,
        }
    }

    pub fn is_zero(self) -> bool {
        self.x == 0 && self.y == 0
    }

    pub fn pairwise_max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    pub fn pairwise_min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
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

pub const MAX_SIZE_FIELD: u32 = ::core::i32::MAX as u32 + 1;

pub const MAX_SIZE: Size = Size {
    x: MAX_SIZE_FIELD,
    y: MAX_SIZE_FIELD,
};

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
    /// Panics if `width` or `width` is greater than `::core::i32::MAX as u32 + 1`.
    pub fn new(width: u32, height: u32) -> Self {
        match Self::try_new(width, height) {
            Err(DimensionTooLargeForSize) => {
                panic!(
                    "Size is too big: ({}, {}). Max is {}.",
                    width, width, MAX_SIZE_FIELD
                );
            }
            Ok(size) => size,
        }
    }

    /// Like new, but const and never panics as it's impossible to construct an invalid size
    pub const fn new_u16(width: u16, height: u16) -> Self {
        Self {
            x: width as u32,
            y: height as u32,
        }
    }

    pub fn from_coord(coord: Coord) -> Result<Self, NegativeDimension> {
        coord.to_size()
    }

    pub fn to_coord(self) -> Result<Coord, DimensionTooLargeForCoord> {
        if self.x > ::core::i32::MAX as u32 || self.y > ::core::i32::MAX as u32 {
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
                panic!("Value is too big: {}. Max is {}.", value, MAX_SIZE_FIELD);
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
                panic!("Value is too big: {}. Max is {}.", value, MAX_SIZE_FIELD);
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

    #[must_use]
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

    pub fn try_set_width(self, width: u32) -> Result<Self, DimensionTooLargeForSize> {
        self.try_set_static::<static_axis::X>(width)
    }

    pub fn try_set_height(self, height: u32) -> Result<Self, DimensionTooLargeForSize> {
        self.try_set_static::<static_axis::Y>(height)
    }

    #[must_use]
    pub fn set_width(self, width: u32) -> Self {
        self.set_static::<static_axis::X>(width)
    }

    #[must_use]
    pub fn set_height(self, height: u32) -> Self {
        self.set_static::<static_axis::Y>(height)
    }

    pub fn try_set_width_in_place(&mut self, width: u32) -> Result<(), DimensionTooLargeForSize> {
        self.try_set_static_in_place::<static_axis::X>(width)
    }

    pub fn try_set_height_in_place(&mut self, height: u32) -> Result<(), DimensionTooLargeForSize> {
        self.try_set_static_in_place::<static_axis::Y>(height)
    }

    pub fn set_width_in_place(&mut self, width: u32) {
        self.set_static_in_place::<static_axis::X>(width)
    }

    pub fn set_height_in_place(&mut self, height: u32) {
        self.set_static_in_place::<static_axis::Y>(height)
    }

    /// Returns the width.
    #[inline]
    pub const fn width(self) -> u32 {
        self.x
    }

    /// Alias for `width`.
    #[inline]
    pub const fn x(self) -> u32 {
        self.x
    }

    /// Returns the height.
    #[inline]
    pub const fn height(self) -> u32 {
        self.y
    }

    /// Alias for `height`.
    #[inline]
    pub const fn y(self) -> u32 {
        self.y
    }

    /// Return the number of cells in a 2D grid of this size.
    pub const fn count(self) -> usize {
        (self.x * self.y) as usize
    }

    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.x
            .checked_sub(rhs.x)
            .and_then(|x| self.y.checked_sub(rhs.y).map(|y| Self::new(x, y)))
    }

    pub fn saturating_sub(self, rhs: Self) -> Self {
        let x = self.x.saturating_sub(rhs.x);
        let y = self.y.saturating_sub(rhs.y);
        Self::new(x, y)
    }

    pub const fn max_field() -> u32 {
        MAX_SIZE_FIELD
    }

    pub const fn max() -> Self {
        MAX_SIZE
    }

    pub fn is_zero(self) -> bool {
        self.x == 0 && self.y == 0
    }

    pub fn is_valid(self, coord: Coord) -> bool {
        coord.is_valid(self)
    }

    pub fn constrain(self, coord: Coord) -> Option<Coord> {
        coord.constrain(self)
    }

    pub fn coord_iter_row_major(self) -> CoordIterRowMajor {
        CoordIterRowMajor::new(self)
    }

    pub fn pairwise_max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    pub fn pairwise_min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
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

pub struct CoordIterRowMajor {
    coord: Coord,
    size: Size,
}

impl CoordIterRowMajor {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            coord: Coord { x: 0, y: 0 },
        }
    }
}

impl Iterator for CoordIterRowMajor {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        if self.coord.y == self.size.height() as i32 {
            return None;
        }
        let coord = self.coord;
        self.coord.x += 1;
        if self.coord.x == self.size.width() as i32 {
            self.coord.x = 0;
            self.coord.y += 1;
        }
        Some(coord)
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

    #[test]
    fn manhattan_dsitance() {
        assert_eq!(Coord::new(-2, 4).manhattan_distance(Coord::new(5, -2)), 13);
    }

    #[test]
    fn rotation() {
        assert_eq!(Coord::new(2, -3).opposite(), Coord::new(-2, 3));
        assert_eq!(Coord::new(2, -3).left90(), Coord::new(-3, -2));
        assert_eq!(Coord::new(2, -3).right90(), Coord::new(3, 2));
        assert_eq!(Coord::new(0, -1).cardinal_left135(), Coord::new(-1, 1));
        assert_eq!(Coord::new(0, -1).cardinal_right135(), Coord::new(1, 1));
        assert_eq!(Coord::new(-1, 0).cardinal_left135(), Coord::new(1, 1));
        assert_eq!(Coord::new(-1, 0).cardinal_right135(), Coord::new(1, -1));
    }
}
