//! References that point into `static` data

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(const_fn)]
#![no_std]

use core::{mem, ops};

/// A value stored in a `static` variable
#[repr(C)]
pub struct Static<T>
where
    T: ?Sized,
{
    data: T,
}

impl<T> Static<T> {
    /// Asserts that this `value` is stored in a `static` variable
    pub const unsafe fn new(value: T) -> Self {
        Static { data: value }
    }

    /// Asserts that the reference `r` points to data stored in a `static`
    /// variable
    pub unsafe fn ref_<'a>(r: &'a T) -> &'a Static<T> {
        mem::transmute(r)
    }

    /// Asserts that the reference `r` points to data stored in a `static`
    /// variable
    pub unsafe fn ref_mut<'a>(r: &'a mut T) -> &'a mut Static<T> {
        mem::transmute(r)
    }
}

impl<T> ops::Deref for Static<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data
    }
}

impl<T> ops::DerefMut for Static<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.data
    }
}
