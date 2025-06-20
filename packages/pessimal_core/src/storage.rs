use std::fmt::Debug;

use crate::Scalar;

pub trait Storage<S: Scalar>: Clone {
    /// The number of elements in the storage.
    fn len(&self) -> usize;

    /// Create a new storage with all elements initialized to zero.
    fn zeros(length: usize) -> Self;

    /// Create a new storage with arbitrary elements.
    fn uninitialized(length: usize) -> Self {
        Self::zeros(length)
    }
}

#[derive(Clone, Debug)]
pub struct BoxStorage<S: Scalar> {
    data: Box<[S]>,
}

impl<S: Scalar> Storage<S> for BoxStorage<S> {
    fn len(&self) -> usize {
        self.data.len()
    }

    fn zeros(length: usize) -> Self {
        Self {
            data: unsafe { Box::new_zeroed_slice(length).assume_init() },
        }
    }
}
