use std::fmt::{Debug, Display};

/**
 * This module implements a generic largest algorithm
* based on generic iterator trait
*/


// We describe the iterator trait that is parameterized by
// the associated type Item, and a function next() which
// returns an option holding the Item
pub trait MyIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// We define an iterable vector of types T, with a trait bound Clone
// as we copy the items for the iterator to return
pub struct MyVector<T>
where T: Clone {
    data: Vec<T>,
    pos: usize,
}

impl<T: Clone> MyVector<T> {
    pub fn new(data: &Vec<T>) -> Self {
        MyVector::<T> {
            data: (*data).clone(),
            pos:0,
        }
    }
}

impl<T: Clone>  MyIterator  for MyVector<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.data.len() {
            None
        } else {
            self.pos = self.pos + 1;
            Some(self.data[self.pos-1].clone())
        }

    }
}


// We define a generic largest function over an iterable type. For comparison we additionally
// enforce the trait bound PartialOrd on the underlying Item type of the iterator.
// For convenience, we also demand that Items be Copy, i.e, every assignment makes a new copy
// of the item (instead of the ownership transfer)
pub fn largest<T: MyIterator<Item: PartialOrd + Copy>>(iterable: &mut T)->Option<T::Item> {
    let mut maxval: Option<T::Item> = None;
    //let mut val = iterable.next();
    while let Some(val) = iterable.next() {
        match maxval {
            None => { maxval = Some(val); }
            Some(x) => { if x < val {
                maxval = Some(val);
            }}
        }
    }
    maxval
}