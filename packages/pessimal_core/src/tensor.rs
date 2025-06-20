use std::{fmt::Debug, marker::PhantomData};

use crate::{Scalar, Storage};

#[derive(Clone, Debug)]
pub struct Tensor<T: Scalar, const RANK: usize, S: Storage<T>> {
    storage: S,
    shape: [usize; RANK],
    strides: [usize; RANK],
    _scalar: PhantomData<T>,
}

impl<T: Scalar, const RANK: usize, S: Storage<T>> Tensor<T, RANK, S> {
    /// Create a new tensor from a raw storage, shape, and strides.
    pub fn new(storage: S, shape: [usize; RANK], strides: [usize; RANK]) -> Self {
        debug_assert_eq!(storage.len(), shape.iter().product(),);
        Self {
            storage,
            shape,
            strides,
            _scalar: PhantomData,
        }
    }

    /// Create a tensor with all elements initialized to zero.
    pub fn zeros(shape: [usize; RANK]) -> Self {
        Self::new(
            S::zeros(shape.iter().product()),
            shape,
            default_strides(shape),
        )
    }

    /// Create a tensor with all elements uninitialized.
    pub fn uninitialized(shape: [usize; RANK]) -> Self {
        Self::new(
            S::uninitialized(shape.iter().product()),
            shape,
            default_strides(shape),
        )
    }

    /// Get the shape of the tensor.
    pub fn shape(&self) -> &[usize; RANK] {
        &self.shape
    }

    /// Get the strides of the tensor.
    pub fn strides(&self) -> &[usize; RANK] {
        &self.strides
    }

    /// Get the storage of the tensor.
    pub fn storage(&self) -> &S {
        &self.storage
    }

    /// Mutably borrow the storage of the tensor.
    pub fn storage_mut(&mut self) -> &mut S {
        &mut self.storage
    }
}

const fn default_strides<const RANK: usize>(shape: [usize; RANK]) -> [usize; RANK] {
    let mut strides = [0; RANK];
    let mut index = 0;
    let mut stride = 1;
    while index < RANK {
        strides[index] = stride;
        stride *= shape[index];
        index += 1;
    }
    strides
}
