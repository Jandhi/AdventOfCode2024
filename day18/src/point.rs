use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T> 
where T : Clone + Copy + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<T, Output = T> + MulAssign<T>
{
    pub x : T,
    pub y : T,
}

impl<T> Point<T> 
where T : Clone + Copy + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<T, Output = T> + MulAssign<T>
{
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

impl<T> Add for Point<T> 
where T : Clone + Copy + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<T, Output = T> + MulAssign<T>
{
    type Output = Point<T>;

    fn add(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> AddAssign for Point<T> 
where T : Clone + Copy + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<T, Output = T> + MulAssign<T>
{
    fn add_assign(&mut self, other: Point<T>) {
        *self = Point {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl<T> Sub for Point<T> 
where T : Clone + Copy + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<T, Output = T> + MulAssign<T>
{
    type Output = Point<T>;

    fn sub(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> SubAssign for Point<T> 
where T : Clone + Copy + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<T, Output = T> + MulAssign<T>
{
    fn sub_assign(&mut self, other: Point<T>) {
        *self = Point {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl<T> Mul<T> for Point<T> 
where T : Clone + Copy + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<T, Output = T> + MulAssign<T>
{
    type Output = Point<T>;

    fn mul(self, other: T) -> Point<T> {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T> MulAssign<T> for Point<T> 
where T : Clone + Copy + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<T, Output = T> + MulAssign<T>
{
    fn mul_assign(&mut self, other: T) {
        *self = Point {
            x: self.x * other,
            y: self.y * other,
        };
    }
}

impl<T1, T2> FromIterator<T1> for Point<T2>
where T2 : Clone + Copy + Add<Output = T2> + AddAssign + Sub<Output = T2> + SubAssign + Mul<T2, Output = T2> + MulAssign<T2>,
    T1: Into<T2>,
{
    fn from_iter<I: IntoIterator<Item = T1>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        Point {
            x: iter.next().unwrap().into(),
            y: iter.next().unwrap().into(),
        }
    }
}

impl<T> Default for Point<T> 
where T : Default + Clone + Copy + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<T, Output = T> + MulAssign<T>
{
    fn default() -> Self {
        Point { x: Default::default(), y: Default::default() }
    }
}