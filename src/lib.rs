//! References that point into `static` data

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use core::ops;

/// A reference that points into `static` data
pub unsafe trait StaticRef<T>: ops::Deref<Target = T> {}

unsafe impl<'a, T> StaticRef<T> for Ref<'a, T> {}
unsafe impl<'a, T> StaticRef<T> for RefMut<'a, T> {}
unsafe impl<T> StaticRef<T> for &'static T {}

/// `&'a T` that points into `static` data
pub struct Ref<'a, T>
where
    T: 'static,
{
    ref_: &'a T,
}

impl<'a, T> Clone for Ref<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T> Copy for Ref<'a, T> {}

impl<'a, T> Ref<'a, T> {
    /// Asserts that `ref_` points into `static` data
    pub unsafe fn new(ref_: &'a T) -> Self {
        Ref { ref_: ref_ }
    }
}

impl<'a, T> ops::Deref for Ref<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.ref_
    }
}

/// `&'a mut T` that points into `static` data
pub struct RefMut<'a, T>
where
    T: 'static,
{
    ref_mut: &'a mut T,
}

impl<'a, T> RefMut<'a, T> {
    /// Asserts that `ref_mut` points into `static` data
    pub unsafe fn new(ref_mut: &'a mut T) -> Self {
        RefMut { ref_mut: ref_mut }
    }
}

impl<'a, T> ops::Deref for RefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.ref_mut
    }
}

impl<'a, T> ops::DerefMut for RefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.ref_mut
    }
}
