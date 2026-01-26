#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Axis {
    X,
    Y,
}

impl Axis {
    pub const fn other(self) -> Self {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::X,
        }
    }
    pub const fn new_icoord(self, this_axis: i32, other_axis: i32) -> ICoord {
        match self {
            Axis::X => ICoord::new(this_axis, other_axis),
            Axis::Y => ICoord::new(other_axis, this_axis),
        }
    }
    pub fn new_ucoord(self, this_axis: u32, other_axis: u32) -> UCoord {
        match self {
            Axis::X => UCoord::new(this_axis, other_axis),
            Axis::Y => UCoord::new(other_axis, this_axis),
        }
    }
}

pub trait StaticAxis: private::Sealed {
    type Other: StaticAxis;
    fn axis() -> Axis;
    fn new_icoord(this_axis: i32, other_axis: i32) -> ICoord;
    fn icoord_get(icoord: ICoord) -> i32;
    fn icoord_get_mut(icoord: &mut ICoord) -> &mut i32;
    fn icoord_with_axis<F: FnMut(i32) -> i32>(icoord: ICoord, f: F) -> ICoord;
    fn icoord_set(icoord: ICoord, value: i32) -> ICoord;
    fn icoord_set_in_place(icoord: &mut ICoord, value: i32);
    fn ucoord_get(ucoord: UCoord) -> u32;
    fn ucoord_with_axis<F: FnMut(u32) -> u32>(ucoord: UCoord, f: F) -> UCoord;
    fn ucoord_set(ucoord: UCoord, value: u32) -> UCoord;
    fn ucoord_set_in_place(ucoord: &mut UCoord, value: u32);
    fn new_ucoord(this_axis: u32, other_axis: u32) -> UCoord;
}

pub mod static_axis {
    pub struct X;
    pub struct Y;
}

fn check_ucoord_limit(value: u32) {
    assert!(
        value <= MAX_UCOORD_FIELD,
        "Value {} is too large for ucoord field",
        value
    );
}

impl StaticAxis for static_axis::X {
    type Other = static_axis::Y;
    fn axis() -> Axis {
        Axis::X
    }
    fn new_icoord(this_axis: i32, other_axis: i32) -> ICoord {
        ICoord::new(this_axis, other_axis)
    }
    fn icoord_get(icoord: ICoord) -> i32 {
        icoord.x
    }
    fn icoord_get_mut(icoord: &mut ICoord) -> &mut i32 {
        &mut icoord.x
    }
    fn icoord_with_axis<F: FnMut(i32) -> i32>(icoord: ICoord, mut f: F) -> ICoord {
        ICoord {
            x: f(icoord.x),
            ..icoord
        }
    }
    fn icoord_set(icoord: ICoord, value: i32) -> ICoord {
        ICoord { x: value, ..icoord }
    }
    fn icoord_set_in_place(icoord: &mut ICoord, value: i32) {
        icoord.x = value
    }
    fn ucoord_get(ucoord: UCoord) -> u32 {
        ucoord.width()
    }
    fn ucoord_with_axis<F: FnMut(u32) -> u32>(ucoord: UCoord, mut f: F) -> UCoord {
        ucoord.set_width(f(ucoord.width()))
    }
    fn ucoord_set(ucoord: UCoord, value: u32) -> UCoord {
        ucoord.set_width(value)
    }
    fn ucoord_set_in_place(ucoord: &mut UCoord, value: u32) {
        ucoord.set_width_in_place(value);
    }
    fn new_ucoord(this_axis: u32, other_axis: u32) -> UCoord {
        UCoord::new(this_axis, other_axis)
    }
}

impl StaticAxis for static_axis::Y {
    type Other = static_axis::X;
    fn axis() -> Axis {
        Axis::Y
    }
    fn new_icoord(this_axis: i32, other_axis: i32) -> ICoord {
        ICoord::new(other_axis, this_axis)
    }
    fn icoord_get(icoord: ICoord) -> i32 {
        icoord.y
    }
    fn icoord_get_mut(icoord: &mut ICoord) -> &mut i32 {
        &mut icoord.y
    }
    fn icoord_with_axis<F: FnMut(i32) -> i32>(icoord: ICoord, mut f: F) -> ICoord {
        ICoord {
            y: f(icoord.y),
            ..icoord
        }
    }
    fn icoord_set(icoord: ICoord, value: i32) -> ICoord {
        ICoord { y: value, ..icoord }
    }
    fn icoord_set_in_place(icoord: &mut ICoord, value: i32) {
        icoord.y = value
    }
    fn ucoord_get(ucoord: UCoord) -> u32 {
        ucoord.height()
    }
    fn ucoord_with_axis<F: FnMut(u32) -> u32>(ucoord: UCoord, mut f: F) -> UCoord {
        ucoord.set_height(f(ucoord.height()))
    }
    fn ucoord_set(ucoord: UCoord, value: u32) -> UCoord {
        ucoord.set_height(value)
    }

    fn ucoord_set_in_place(ucoord: &mut UCoord, value: u32) {
        ucoord.set_height_in_place(value);
    }

    fn new_ucoord(this_axis: u32, other_axis: u32) -> UCoord {
        UCoord::new(other_axis, this_axis)
    }
}

mod private {
    pub trait Sealed {}

    impl Sealed for super::static_axis::X {}
    impl Sealed for super::static_axis::Y {}
}

/// General purpose icoordinate
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct ICoord {
    pub x: i32,
    pub y: i32,
}

impl core::fmt::Display for ICoord {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
pub struct NegativeDimension;

#[derive(Debug)]
pub struct DimensionTooLargeForUCoord;

#[derive(Debug)]
pub struct DimensionTooLargeForICoord;

impl ICoord {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub const fn from_ucoord(ucoord: UCoord) -> Self {
        ucoord.to_icoord()
    }
    #[cfg(feature = "rand")]
    pub fn random_within<R: rand::Rng>(ucoord: UCoord, rng: &mut R) -> Self {
        let x = rng.random_range(0..ucoord.width() as i32);
        let y = rng.random_range(0..ucoord.height() as i32);
        Self { x, y }
    }
    pub fn to_ucoord(self) -> UCoord {
        assert!(
            self.x >= 0 && self.y >= 0,
            "ICoord {} can't be converted to UCoord as it contains a negative component.",
            self
        );
        UCoord(self)
    }
    const fn normalize_part(value: i32, ucoord: u32) -> i32 {
        let value = value % ucoord as i32;
        if value < 0 {
            value + ucoord as i32
        } else {
            value
        }
    }
    pub const fn normalize(self, ucoord: UCoord) -> Self {
        Self {
            x: Self::normalize_part(self.x, ucoord.width()),
            y: Self::normalize_part(self.y, ucoord.height()),
        }
    }
    pub const fn is_valid(self, ucoord: UCoord) -> bool {
        if self.x < 0 || self.y < 0 {
            return false;
        }
        let x = self.x as u32;
        let y = self.y as u32;
        x < ucoord.width() && y < ucoord.height()
    }
    pub const fn constrain(mut self, ucoord: UCoord) -> Option<Self> {
        if self.x < 0 {
            self.x = 0;
        }
        if self.y < 0 {
            self.y = 0;
        }
        let max_x = if let Some(max_x) = ucoord.width().checked_sub(1) {
            max_x as i32
        } else {
            return None;
        };
        if self.x > max_x {
            self.x = max_x;
        }
        let max_y = if let Some(max_y) = ucoord.height().checked_sub(1) {
            max_y as i32
        } else {
            return None;
        };
        if self.y > max_y {
            self.y = max_y;
        }
        Some(self)
    }
    pub const fn get(self, axis: Axis) -> i32 {
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

    #[must_use]
    pub const fn set(self, axis: Axis, value: i32) -> Self {
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
    pub const fn new_axis(this_axis: i32, other_axis: i32, axis: Axis) -> Self {
        axis.new_icoord(this_axis, other_axis)
    }
    pub fn get_static<A: StaticAxis>(self) -> i32 {
        A::icoord_get(self)
    }
    pub fn get_static_mut<A: StaticAxis>(&mut self) -> &mut i32 {
        A::icoord_get_mut(self)
    }
    pub fn with_static_axis<A: StaticAxis, F: FnMut(i32) -> i32>(self, f: F) -> Self {
        A::icoord_with_axis(self, f)
    }
    pub fn set_static<A: StaticAxis>(self, value: i32) -> Self {
        A::icoord_set(self, value)
    }
    pub fn set_static_in_place<A: StaticAxis>(&mut self, value: i32) {
        A::icoord_set_in_place(self, value)
    }
    pub fn new_static_axis<A: StaticAxis>(this_axis: i32, other_axis: i32) -> Self {
        A::new_icoord(this_axis, other_axis)
    }
    #[must_use]
    pub const fn set_x(self, x: i32) -> Self {
        Self { x, ..self }
    }
    #[must_use]
    pub const fn set_y(self, y: i32) -> Self {
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

    pub const fn is_zero(self) -> bool {
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

    pub const fn transpose(self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }
}

/// Shorthand for ICoord::new
pub const fn icoord(x: i32, y: i32) -> ICoord {
    ICoord::new(x, y)
}

impl From<(i32, i32)> for ICoord {
    fn from((x, y): (i32, i32)) -> Self {
        ICoord::new(x, y)
    }
}

impl From<ICoord> for (i32, i32) {
    fn from(ICoord { x, y }: ICoord) -> Self {
        (x, y)
    }
}

impl From<[i32; 2]> for ICoord {
    fn from(array: [i32; 2]) -> Self {
        ICoord::new(array[0], array[1])
    }
}

impl From<ICoord> for [i32; 2] {
    fn from(ICoord { x, y }: ICoord) -> Self {
        [x, y]
    }
}

// Internally a UCoord is an ICoord, but we prevent UCoords from being created with negative
// dimensions.
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct UCoord(ICoord);

pub const MAX_UCOORD_FIELD: u32 = ::core::i32::MAX as u32;

pub const MAX_UCOORD: UCoord = UCoord(ICoord {
    x: MAX_UCOORD_FIELD as i32,
    y: MAX_UCOORD_FIELD as i32,
});

impl UCoord {
    const fn new_unchecked(width: u32, height: u32) -> Self {
        Self(ICoord {
            x: width as i32,
            y: height as i32,
        })
    }

    /// Creates a new `UCoord`.
    /// Panics if `width` or `height` is greater than `::core::i32::MAX`.
    pub fn new(width: u32, height: u32) -> Self {
        check_ucoord_limit(width);
        check_ucoord_limit(height);
        Self::new_unchecked(width, height)
    }

    /// Like new, but const and never panics as it's impossible to construct an invalid ucoord
    pub const fn new_u16(width: u16, height: u16) -> Self {
        Self(ICoord {
            x: width as i32,
            y: height as i32,
        })
    }

    pub fn from_icoord(icoord: ICoord) -> Self {
        icoord.to_ucoord()
    }

    pub const fn to_icoord(self) -> ICoord {
        self.0
    }

    pub const fn get(self, axis: Axis) -> u32 {
        match axis {
            Axis::X => self.width(),
            Axis::Y => self.height(),
        }
    }

    pub fn with_axis<F: FnMut(u32) -> u32>(self, axis: Axis, mut f: F) -> Self {
        match axis {
            Axis::X => self.set_width(f(self.width())),
            Axis::Y => self.set_height(f(self.height())),
        }
    }

    #[must_use]
    pub fn set(self, axis: Axis, value: u32) -> Self {
        check_ucoord_limit(value);
        match axis {
            Axis::X => self.set_width(value),
            Axis::Y => self.set_height(value),
        }
    }

    pub fn set_in_place(&mut self, axis: Axis, value: u32) {
        check_ucoord_limit(value);
        match axis {
            Axis::X => self.set_width_in_place(value),
            Axis::Y => self.set_height_in_place(value),
        }
    }

    pub fn new_axis(this_axis: u32, other_axis: u32, axis: Axis) -> Self {
        let (width, height) = match axis {
            Axis::X => (this_axis, other_axis),
            Axis::Y => (other_axis, this_axis),
        };
        Self::new(width, height)
    }

    pub fn get_static<A: StaticAxis>(self) -> u32 {
        A::ucoord_get(self)
    }
    pub fn with_static_axis<A: StaticAxis, F: FnMut(u32) -> u32>(self, f: F) -> Self {
        A::ucoord_with_axis(self, f)
    }

    #[must_use]
    pub fn set_static<A: StaticAxis>(self, value: u32) -> Self {
        A::ucoord_set(self, value)
    }

    pub fn set_static_in_place<A: StaticAxis>(&mut self, value: u32) {
        A::ucoord_set_in_place(self, value)
    }

    pub fn new_static_axis<A: StaticAxis>(this_axis: u32, other_axis: u32) -> Self {
        A::new_ucoord(this_axis, other_axis)
    }

    #[must_use]
    pub fn set_width(mut self, width: u32) -> Self {
        check_ucoord_limit(width);
        self.0.x = width as i32;
        self
    }

    #[must_use]
    pub fn set_height(mut self, height: u32) -> Self {
        check_ucoord_limit(height);
        self.0.y = height as i32;
        self
    }

    pub fn set_width_in_place(&mut self, width: u32) {
        check_ucoord_limit(width);
        self.0.x = width as i32;
    }

    pub fn set_height_in_place(&mut self, height: u32) {
        check_ucoord_limit(height);
        self.0.y = height as i32;
    }

    /// Returns the width.
    #[inline]
    pub const fn width(self) -> u32 {
        self.0.x as u32
    }

    /// Returns the height.
    #[inline]
    pub const fn height(self) -> u32 {
        self.0.y as u32
    }

    /// Return the number of cells in a 2D grid of this size.
    pub const fn count(self) -> usize {
        (self.width() * self.height()) as usize
    }

    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.width().checked_sub(rhs.width()).and_then(|width| {
            self.height()
                .checked_sub(rhs.height())
                .map(|height| Self::new(width, height))
        })
    }

    pub fn saturating_sub(self, rhs: Self) -> Self {
        let width = self.width().saturating_sub(rhs.width());
        let height = self.height().saturating_sub(rhs.height());
        Self::new(width, height)
    }

    pub const fn max_field() -> u32 {
        MAX_UCOORD_FIELD
    }

    pub const fn max() -> Self {
        MAX_UCOORD
    }

    pub const fn is_zero(self) -> bool {
        self.width() == 0 && self.height() == 0
    }

    pub const fn is_valid(self, icoord: ICoord) -> bool {
        icoord.is_valid(self)
    }

    pub const fn constrain(self, icoord: ICoord) -> Option<ICoord> {
        icoord.constrain(self)
    }

    pub const fn icoord_iter_row_major(self) -> ICoordIterRowMajor {
        ICoordIterRowMajor::new(self)
    }

    pub fn pairwise_max(self, other: Self) -> Self {
        Self::new_unchecked(
            self.width().max(other.width()),
            self.height().max(other.height()),
        )
    }

    pub fn pairwise_min(self, other: Self) -> Self {
        Self::new_unchecked(
            self.width().min(other.width()),
            self.height().min(other.height()),
        )
    }

    pub const fn transpose(self) -> Self {
        Self::new_unchecked(self.height(), self.width())
    }

    pub const fn is_empty(self) -> bool {
        self.width() == 0 || self.height() == 0
    }

    pub const fn is_on_edge(self, ICoord { x, y }: ICoord) -> bool {
        ((x == 0 || x == self.width() as i32 - 1) && y >= 0 && y < self.height() as i32)
            || ((y == 0 || y == self.height() as i32 - 1) && x >= 0 && x < self.width() as i32)
    }

    pub fn edge_iter(self) -> EdgeIter {
        edge_iter::make_iter(self)
    }
}

/// Shorthand for UCoord::new
pub fn ucoord(width: u32, height: u32) -> UCoord {
    UCoord::new(width, height)
}

/// Shorthand for UCoord::new_u16
pub const fn ucoord_u16(width: u16, height: u16) -> UCoord {
    UCoord::new_u16(width, height)
}

mod edge_iter {
    use super::{ICoord, ICoordIterRowMajor, UCoord};
    use core::{
        iter::{Chain, Rev},
        ops::Range,
    };

    struct Top {
        x_range: Range<i32>,
    }

    impl Iterator for Top {
        type Item = ICoord;
        fn next(&mut self) -> Option<Self::Item> {
            self.x_range.next().map(|x| ICoord { x, y: 0 })
        }
    }

    struct Right {
        y_range: Range<i32>,
        x: i32,
    }

    impl Iterator for Right {
        type Item = ICoord;
        fn next(&mut self) -> Option<Self::Item> {
            self.y_range.next().map(|y| ICoord { x: self.x, y })
        }
    }

    struct Bottom {
        x_range: Rev<Range<i32>>,
        y: i32,
    }

    impl Iterator for Bottom {
        type Item = ICoord;
        fn next(&mut self) -> Option<Self::Item> {
            self.x_range.next().map(|x| ICoord { x, y: self.y })
        }
    }

    struct Left {
        y_range: Rev<Range<i32>>,
    }

    impl Iterator for Left {
        type Item = ICoord;
        fn next(&mut self) -> Option<Self::Item> {
            self.y_range.next().map(|y| ICoord { x: 0, y })
        }
    }

    type TwoDimensional = Chain<Chain<Chain<Top, Right>, Bottom>, Left>;

    enum IterPrivate {
        ZeroDimensional,
        OneDimensional(ICoordIterRowMajor),
        TwoDimensional(TwoDimensional),
    }

    impl Iterator for IterPrivate {
        type Item = ICoord;
        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::ZeroDimensional => None,
                Self::OneDimensional(iter) => iter.next(),
                Self::TwoDimensional(iter) => iter.next(),
            }
        }
    }

    pub struct Iter(IterPrivate);

    impl Iterator for Iter {
        type Item = ICoord;
        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }

    pub fn make_iter(ucoord: UCoord) -> Iter {
        let iter_private = if ucoord.is_empty() {
            IterPrivate::ZeroDimensional
        } else if ucoord.width() == 1 || ucoord.height() == 1 {
            IterPrivate::OneDimensional(ucoord.icoord_iter_row_major())
        } else {
            let top = Top {
                x_range: 0..ucoord.width() as i32 - 1,
            };
            let right = Right {
                y_range: 0..ucoord.height() as i32 - 1,
                x: ucoord.width() as i32 - 1,
            };
            let bottom = Bottom {
                x_range: (1..ucoord.width() as i32).rev(),
                y: ucoord.height() as i32 - 1,
            };
            let left = Left {
                y_range: (1..ucoord.height() as i32).rev(),
            };
            let all = top.chain(right).chain(bottom).chain(left);
            IterPrivate::TwoDimensional(all)
        };
        Iter(iter_private)
    }
}
pub use edge_iter::Iter as EdgeIter;

impl From<(u32, u32)> for UCoord {
    fn from((x, y): (u32, u32)) -> Self {
        UCoord::new(x, y)
    }
}

impl From<[u32; 2]> for UCoord {
    fn from(array: [u32; 2]) -> Self {
        UCoord::new(array[0], array[1])
    }
}

pub struct ICoordIterRowMajor {
    icoord: ICoord,
    ucoord: UCoord,
}

impl ICoordIterRowMajor {
    pub const fn new(ucoord: UCoord) -> Self {
        Self {
            ucoord,
            icoord: ICoord { x: 0, y: 0 },
        }
    }
}

impl Iterator for ICoordIterRowMajor {
    type Item = ICoord;
    fn next(&mut self) -> Option<Self::Item> {
        if self.icoord.y == self.ucoord.height() as i32 {
            return None;
        }
        let icoord = self.icoord;
        self.icoord.x += 1;
        if self.icoord.x == self.ucoord.width() as i32 {
            self.icoord.x = 0;
            self.icoord.y += 1;
        }
        Some(icoord)
    }
}

#[cfg(test)]
mod test {
    use super::{ICoord, UCoord};

    #[test]
    fn normalize() {
        assert_eq!(
            ICoord::new(5, 2).normalize(UCoord::new(2, 3)),
            ICoord::new(1, 2)
        );
        assert_eq!(
            ICoord::new(-4, 3).normalize(UCoord::new(3, 1)),
            ICoord::new(2, 0)
        );
    }

    #[test]
    fn manhattan_dsitance() {
        assert_eq!(
            ICoord::new(-2, 4).manhattan_distance(ICoord::new(5, -2)),
            13
        );
    }

    #[test]
    fn rotation() {
        assert_eq!(ICoord::new(2, -3).opposite(), ICoord::new(-2, 3));
        assert_eq!(ICoord::new(2, -3).left90(), ICoord::new(-3, -2));
        assert_eq!(ICoord::new(2, -3).right90(), ICoord::new(3, 2));
        assert_eq!(ICoord::new(0, -1).cardinal_left135(), ICoord::new(-1, 1));
        assert_eq!(ICoord::new(0, -1).cardinal_right135(), ICoord::new(1, 1));
        assert_eq!(ICoord::new(-1, 0).cardinal_left135(), ICoord::new(1, 1));
        assert_eq!(ICoord::new(-1, 0).cardinal_right135(), ICoord::new(1, -1));
    }

    #[test]
    fn edge() {
        assert!(UCoord::new(3, 5).is_on_edge(ICoord::new(0, 0)));
        assert!(UCoord::new(3, 5).is_on_edge(ICoord::new(2, 0)));
        assert!(UCoord::new(3, 5).is_on_edge(ICoord::new(0, 3)));
        assert!(UCoord::new(3, 5).is_on_edge(ICoord::new(0, 4)));
        assert!(UCoord::new(3, 5).is_on_edge(ICoord::new(1, 4)));
        assert!(!UCoord::new(3, 5).is_on_edge(ICoord::new(1, 5)));
    }

    #[test]
    #[cfg(feature = "std")]
    fn edge_iter() {
        use std::collections::BTreeSet;
        fn test(ucoord: UCoord) {
            let brute_forced = ucoord
                .icoord_iter_row_major()
                .filter(|&c| ucoord.is_on_edge(c))
                .collect::<Vec<_>>();
            let via_iter = ucoord.edge_iter().collect::<Vec<_>>();
            assert_eq!(brute_forced.len(), via_iter.len());
            assert_eq!(
                brute_forced.iter().cloned().collect::<BTreeSet<_>>(),
                via_iter.iter().cloned().collect::<BTreeSet<_>>(),
            );
        }
        test(UCoord::new(0, 0));
        test(UCoord::new(1, 1));
        test(UCoord::new(1, 3));
        test(UCoord::new(3, 1));
        test(UCoord::new(2, 2));
        test(UCoord::new(3, 5));
    }
}
