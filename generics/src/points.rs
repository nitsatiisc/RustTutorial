use std::fmt::{Display, Formatter, Write};
use std::ops::{Add, Mul, Sub};

/**
 * In this file we talk about traits.
 * Traits in Rust characterize a set of types that implement a given set of functions (signature)
 * This allows us to write functions which conform to traits than to individual types
 * Traits themselves form a hierarchy, where a trait can derive from a supertrait
 * As an example, Clone is a supertrait of Copy, i.e, a type that is Copy is also Clone.
 *
 * Common Traits:
 * Display :- this is used by printing routines to print an object.
*/

#[derive(Debug, Clone, Copy)]
pub struct SimplePoint {
    x: i32,
    y: i32,
}

impl SimplePoint {
    // this is a static function
    pub fn new(x: i32, y:i32) -> Self {
        SimplePoint {
            x,
            y,
        }
    }

    // this is a class method that modifies the instance
    // hint: takes a mutable reference to self
    pub fn double(&mut self) {
        self.x = 2*self.x;
        self.y = 2*self.y;
    }

}

// Let's implement addition for simple point by implementing the Add trait
impl Add for SimplePoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Display for SimplePoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "( {},{} )", self.x, self.y)
    }
}

// ---------------------------------- Now we make a generic point --------------------------------------//

#[derive(Clone,Debug, Copy)]
pub struct Point<T>
{
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point::<T> {
            x,
            y,
        }
    }
}

impl<T> Point<T>
where T: Clone + Add<Output=T> {
    pub fn double(&mut self) {
        self.x = self.x.clone() + self.x.clone();
        self.y = self.y.clone() + self.y.clone();
    }
}

// We can define addition on points when the type
// T safisfies trait bound Add<Output=T>, i.e, T can be added
// to itself with result being T again.
// As add method in Add trait takes ownership of the object, in most cases
// we want T to implement Copy as well.
impl<T: Add<Output=T>> Add for Point<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}


// We can implement display for points when type
// T satisfies trait bound Display
impl<T: Display> Display for Point<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "( {}, {})", self.x, self.y)
    }
}