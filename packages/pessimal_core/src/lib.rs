#![feature(new_zeroed_alloc)]

pub mod scalar;
pub mod storage;
pub mod tensor;

pub use scalar::Scalar;
pub use storage::Storage;
pub use tensor::Tensor;