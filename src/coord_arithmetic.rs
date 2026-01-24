use crate::coord::{ICoord, UCoord};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl Neg for ICoord {
    type Output = ICoord;
    fn neg(self) -> Self::Output {
        ICoord {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<'a> Neg for &'a ICoord {
    type Output = ICoord;
    fn neg(self) -> Self::Output {
        ICoord {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add for ICoord {
    type Output = ICoord;
    fn add(self, ICoord { x, y }: ICoord) -> Self::Output {
        ICoord {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl<'a> Add<ICoord> for &'a ICoord {
    type Output = ICoord;
    fn add(self, ICoord { x, y }: ICoord) -> Self::Output {
        ICoord {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl<'a> Add<&'a ICoord> for ICoord {
    type Output = ICoord;
    fn add(self, &ICoord { x, y }: &'a ICoord) -> Self::Output {
        ICoord {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl<'a, 'b> Add<&'a ICoord> for &'b ICoord {
    type Output = ICoord;
    fn add(self, &ICoord { x, y }: &'a ICoord) -> Self::Output {
        ICoord {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl Add<UCoord> for ICoord {
    type Output = ICoord;
    fn add(self, ucoord: UCoord) -> Self::Output {
        ICoord {
            x: self.x + ucoord.width() as i32,
            y: self.y + ucoord.height() as i32,
        }
    }
}

impl<'a> Add<UCoord> for &'a ICoord {
    type Output = ICoord;
    fn add(self, ucoord: UCoord) -> Self::Output {
        ICoord {
            x: self.x + ucoord.width() as i32,
            y: self.y + ucoord.height() as i32,
        }
    }
}

impl<'a> Add<&'a UCoord> for ICoord {
    type Output = ICoord;
    fn add(self, ucoord: &'a UCoord) -> Self::Output {
        ICoord {
            x: self.x + ucoord.width() as i32,
            y: self.y + ucoord.height() as i32,
        }
    }
}

impl<'a, 'b> Add<&'a UCoord> for &'b ICoord {
    type Output = ICoord;
    fn add(self, ucoord: &'a UCoord) -> Self::Output {
        ICoord {
            x: self.x + ucoord.width() as i32,
            y: self.y + ucoord.height() as i32,
        }
    }
}

impl Add<ICoord> for UCoord {
    type Output = ICoord;
    fn add(self, ICoord { x, y }: ICoord) -> Self::Output {
        ICoord {
            x: self.width() as i32 + x,
            y: self.height() as i32 + y,
        }
    }
}

impl<'a> Add<ICoord> for &'a UCoord {
    type Output = ICoord;
    fn add(self, ICoord { x, y }: ICoord) -> Self::Output {
        ICoord {
            x: self.width() as i32 + x,
            y: self.height() as i32 + y,
        }
    }
}

impl<'a> Add<&'a ICoord> for UCoord {
    type Output = ICoord;
    fn add(self, &ICoord { x, y }: &'a ICoord) -> Self::Output {
        ICoord {
            x: self.width() as i32 + x,
            y: self.height() as i32 + y,
        }
    }
}

impl<'a, 'b> Add<&'a ICoord> for &'b UCoord {
    type Output = ICoord;
    fn add(self, &ICoord { x, y }: &'a ICoord) -> Self::Output {
        ICoord {
            x: self.width() as i32 + x,
            y: self.height() as i32 + y,
        }
    }
}

impl Add for UCoord {
    type Output = UCoord;
    fn add(self, ucoord: UCoord) -> Self::Output {
        UCoord::new(
            self.width() + ucoord.width(),
            self.height() + ucoord.height(),
        )
    }
}

impl<'a> Add<UCoord> for &'a UCoord {
    type Output = UCoord;
    fn add(self, ucoord: UCoord) -> Self::Output {
        UCoord::new(
            self.width() + ucoord.width(),
            self.height() + ucoord.height(),
        )
    }
}

impl<'a> Add<&'a UCoord> for UCoord {
    type Output = UCoord;
    fn add(self, ucoord: &'a UCoord) -> Self::Output {
        UCoord::new(
            self.width() + ucoord.width(),
            self.height() + ucoord.height(),
        )
    }
}

impl<'a, 'b> Add<&'a UCoord> for &'b UCoord {
    type Output = UCoord;
    fn add(self, ucoord: &'a UCoord) -> Self::Output {
        UCoord::new(
            self.width() + ucoord.width(),
            self.height() + ucoord.height(),
        )
    }
}

impl<T> AddAssign<T> for ICoord
where
    ICoord: Add<T, Output = ICoord>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl Sub for ICoord {
    type Output = ICoord;
    fn sub(self, ICoord { x, y }: ICoord) -> Self::Output {
        ICoord {
            x: self.x - x,
            y: self.y - y,
        }
    }
}

impl<'a> Sub<ICoord> for &'a ICoord {
    type Output = ICoord;
    fn sub(self, ICoord { x, y }: ICoord) -> Self::Output {
        ICoord {
            x: self.x - x,
            y: self.y - y,
        }
    }
}

impl<'a> Sub<&'a ICoord> for ICoord {
    type Output = ICoord;
    fn sub(self, &ICoord { x, y }: &'a ICoord) -> Self::Output {
        ICoord {
            x: self.x - x,
            y: self.y - y,
        }
    }
}

impl<'a, 'b> Sub<&'a ICoord> for &'b ICoord {
    type Output = ICoord;
    fn sub(self, &ICoord { x, y }: &'a ICoord) -> Self::Output {
        ICoord {
            x: self.x - x,
            y: self.y - y,
        }
    }
}

impl Sub<UCoord> for ICoord {
    type Output = ICoord;
    fn sub(self, ucoord: UCoord) -> Self::Output {
        ICoord {
            x: self.x - ucoord.width() as i32,
            y: self.y - ucoord.height() as i32,
        }
    }
}

impl<'a> Sub<UCoord> for &'a ICoord {
    type Output = ICoord;
    fn sub(self, ucoord: UCoord) -> Self::Output {
        ICoord {
            x: self.x - ucoord.width() as i32,
            y: self.y - ucoord.height() as i32,
        }
    }
}

impl<'a> Sub<&'a UCoord> for ICoord {
    type Output = ICoord;
    fn sub(self, ucoord: &'a UCoord) -> Self::Output {
        ICoord {
            x: self.x - ucoord.width() as i32,
            y: self.y - ucoord.height() as i32,
        }
    }
}

impl<'a, 'b> Sub<&'a UCoord> for &'b ICoord {
    type Output = ICoord;
    fn sub(self, ucoord: &'a UCoord) -> Self::Output {
        ICoord {
            x: self.x - ucoord.width() as i32,
            y: self.y - ucoord.height() as i32,
        }
    }
}

impl Sub<ICoord> for UCoord {
    type Output = ICoord;
    fn sub(self, ICoord { x, y }: ICoord) -> Self::Output {
        ICoord {
            x: self.width() as i32 - x,
            y: self.height() as i32 - y,
        }
    }
}

impl<'a> Sub<ICoord> for &'a UCoord {
    type Output = ICoord;
    fn sub(self, ICoord { x, y }: ICoord) -> Self::Output {
        ICoord {
            x: self.width() as i32 - x,
            y: self.height() as i32 - y,
        }
    }
}

impl<'a> Sub<&'a ICoord> for UCoord {
    type Output = ICoord;
    fn sub(self, &ICoord { x, y }: &'a ICoord) -> Self::Output {
        ICoord {
            x: self.width() as i32 - x,
            y: self.height() as i32 - y,
        }
    }
}

impl<'a, 'b> Sub<&'a ICoord> for &'b UCoord {
    type Output = ICoord;
    fn sub(self, &ICoord { x, y }: &'a ICoord) -> Self::Output {
        ICoord {
            x: self.width() as i32 - x,
            y: self.height() as i32 - y,
        }
    }
}

impl Sub for UCoord {
    type Output = UCoord;
    fn sub(self, ucoord: UCoord) -> Self::Output {
        UCoord::new(
            self.width() - ucoord.width(),
            self.height() - ucoord.height(),
        )
    }
}

impl<'a> Sub<UCoord> for &'a UCoord {
    type Output = UCoord;
    fn sub(self, ucoord: UCoord) -> Self::Output {
        UCoord::new(
            self.width() - ucoord.width(),
            self.height() - ucoord.height(),
        )
    }
}

impl<'a> Sub<&'a UCoord> for UCoord {
    type Output = UCoord;
    fn sub(self, ucoord: &'a UCoord) -> Self::Output {
        UCoord::new(
            self.width() - ucoord.width(),
            self.height() - ucoord.height(),
        )
    }
}

impl<'a, 'b> Sub<&'a UCoord> for &'b UCoord {
    type Output = UCoord;
    fn sub(self, ucoord: &'a UCoord) -> Self::Output {
        UCoord::new(
            self.width() - ucoord.width(),
            self.height() - ucoord.height(),
        )
    }
}

impl<T> SubAssign<T> for ICoord
where
    ICoord: Sub<T, Output = ICoord>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

impl Mul<i32> for ICoord {
    type Output = ICoord;
    fn mul(self, rhs: i32) -> Self::Output {
        ICoord {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<'a> Mul<i32> for &'a ICoord {
    type Output = ICoord;
    fn mul(self, rhs: i32) -> Self::Output {
        ICoord {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> MulAssign<T> for ICoord
where
    ICoord: Mul<T, Output = ICoord>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl Mul<u32> for UCoord {
    type Output = UCoord;
    fn mul(self, rhs: u32) -> Self::Output {
        UCoord::new(self.width() * rhs, self.height() * rhs)
    }
}

impl<'a> Mul<u32> for &'a UCoord {
    type Output = UCoord;
    fn mul(self, rhs: u32) -> Self::Output {
        UCoord::new(self.width() * rhs, self.height() * rhs)
    }
}

impl<T> MulAssign<T> for UCoord
where
    UCoord: Mul<T, Output = UCoord>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl Div<i32> for ICoord {
    type Output = ICoord;
    fn div(self, rhs: i32) -> Self::Output {
        ICoord {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<'a> Div<i32> for &'a ICoord {
    type Output = ICoord;
    fn div(self, rhs: i32) -> Self::Output {
        ICoord {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> DivAssign<T> for ICoord
where
    ICoord: Div<T, Output = ICoord>,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl Div<u32> for UCoord {
    type Output = UCoord;
    fn div(self, rhs: u32) -> Self::Output {
        UCoord::new(self.width() / rhs, self.height() / rhs)
    }
}

impl<'a> Div<u32> for &'a UCoord {
    type Output = UCoord;
    fn div(self, rhs: u32) -> Self::Output {
        UCoord::new(self.width() / rhs, self.height() / rhs)
    }
}

impl<T> DivAssign<T> for UCoord
where
    UCoord: Div<T, Output = UCoord>,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod test {
    use crate::coord::{ICoord, UCoord};

    #[test]
    fn arithmetic() {
        let mut a = ICoord::new(0, 0);
        let _ = a + ICoord::new(0, 0);
        let _ = &a + ICoord::new(0, 0);
        let _ = a + &ICoord::new(0, 0);
        let _ = &a + &ICoord::new(0, 0);
        let _ = a + UCoord::new(0, 0);
        let _ = &a + UCoord::new(0, 0);
        let _ = a + &UCoord::new(0, 0);
        let _ = &a + &UCoord::new(0, 0);
        let _ = UCoord::new(0, 0) + a;
        let _ = &UCoord::new(0, 0) + a;
        let _ = UCoord::new(0, 0) + &a;
        let _ = &UCoord::new(0, 0) + &a;
        let _ = a += UCoord::new(0, 0);
        let _ = a += ICoord::new(0, 0);
        let _ = a += &UCoord::new(0, 0);
        let _ = a += &ICoord::new(0, 0);
    }
}
