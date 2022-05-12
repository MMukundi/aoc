use std::cmp::PartialOrd;
use std::ops::{Add, Div, Sub};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Default, PartialEq, Hash, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn same(val: i32) -> Self {
        Point { x: val, y: val }
    }
    pub fn in_range(&self, a: &Point, b: &Point) -> bool {
        a.x <= self.x && self.x < b.x && a.y <= self.y && self.y < b.y
    }
    pub fn zero() -> Self {
        Self::same(0)
    }

    fn taxi_cab(&self) -> i32 {
        self.x + self.y
    }
    pub fn dot(&self, other: &Self) -> i32 {
        (self.x * other.x) + (self.y * other.y)
    }
    pub fn gcd(&self) -> Self {
        if self.x * self.y == 0 {
            *self
        } else {
            let mut a = self.x;
            let mut b = self.y;
            loop {
                if b == 0 {
                    break;
                }
                let temp = b;
                b = a % b;
                a = temp;
            }
            *self / Point::same(a)
        }
    }
    pub fn to_step(&self) -> Self {
        Point {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
}
impl FromStr for Point {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.split(',').map(|s| s.parse::<i32>().unwrap_or(0));
        Ok(Point {
            x: coords.next().unwrap(),
            y: coords.next().unwrap(),
        })
    }
}
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Div for Point {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.taxi_cab().partial_cmp(&other.taxi_cab())
    }
}

// ----------------------------------------------------
pub trait SafeTrait {
    fn safe_method(&self) -> i32;
}

pub trait UnsafeTrait: SafeTrait {
    fn unsafe_method(value: i32) -> Self;
}

impl std::fmt::Debug for dyn SafeTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "SAFE::{}", self.safe_method())
    }
}

pub struct UnsafeStruct(i32);
impl SafeTrait for UnsafeStruct {
    fn safe_method(&self) -> i32 {
        self.0
    }
}
impl UnsafeTrait for UnsafeStruct {
    fn unsafe_method(value: i32) -> Self {
        Self(value)
    }
}
#[cfg(test)]
mod tests {
    use super::UnsafeStruct;
    use crate::utils::point::SafeTrait;
    #[test]
    fn main() {
        println!(
            "Calling safe code from an unsafe struct using an unsafe trait: {:?}!",
            UnsafeStruct(100).safe_method()
        );
    }
}
